use cosmwasm_std::{entry_point, to_binary, Binary, Env, StdError, StdResult, Storage, Uint128, Addr, CanonicalAddr, DepsMut, MessageInfo, Response, Deps, Attribute};
use std::ops::Sub;
use sha2::{Sha256, Digest};
use std::str::FromStr;
use hex::{encode as HexEncode, decode as HexDecode};
use obi::OBIDecode;

use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::struct_types::{BlockDetail, CandidateBlockDetail, ValidatorWithPower, VerifyOracleDataResponse, GetValidatorPowerResponse};
use crate::state::{
    validators_power,
    total_validator_power,
    block_details,
    owner,
    block_details_read,
    total_validator_power_read,
    validators_power_read,
    candidate_block_details,
    total_validator_power_last_updated_read,
    total_validator_power_last_updated,
    candidate_block_details_read,
    verified_results,
    verified_results_read,
    owner_read
};
use crate::libraries::{result_codec, tm_signature, iavl_merkle_path};
use crate::libraries::utils;
use crate::libraries::obi::{RelayCandidateBlockInput, AppendSignatureInput, VerifyAndSaveResultInput};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    owner(deps.storage).save(&deps.api.addr_canonicalize(&info.sender.as_str())?)?;
    total_validator_power(deps.storage).save(&Uint128::from(0u64))?;
    total_validator_power_last_updated(deps.storage).save(&0u64)?;
    for idx in 0usize..msg.validators.len() {
        let validator = &msg.validators[idx];
        match validators_power_read(deps.storage).get(&validator.addr.as_slice()) {
            Some(_data) => return Err(StdError::generic_err("DUPLICATION_IN_INITIAL_VALIDATOR_SET")),
            _ => {
                validators_power(deps.storage).set(&validator.addr.as_slice(), validator.power.to_string().as_bytes());
                let old_total_validator_power = total_validator_power_read(deps.storage).load().unwrap();
                total_validator_power(deps.storage).save(&(old_total_validator_power + validator.power))?;
            }
        }
    }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::TransferOwnership { new_owner } => try_transfer_ownership(deps, info, new_owner),
        ExecuteMsg::UpdateValidatorsPower { block_height, validators } => try_update_validators_power(deps, info, block_height, validators),
        ExecuteMsg::RelayCandidateBlock { data } => try_relay_candidate_block(deps, info, data),
        ExecuteMsg::AppendSignature { data } => try_append_signature(deps, info, data),
        ExecuteMsg::VerifyAndSaveResult { data } => try_verify_and_save_result(deps, data),
        ExecuteMsg::RemoveCandidateBlock { block_height } => try_remove_candidate_block(deps, info, block_height),
    }
}

pub fn try_transfer_ownership(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: Addr,
) -> StdResult<Response> {
    if deps.api.addr_canonicalize(&info.sender.as_str())? != owner(deps.storage).load()? {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    owner(deps.storage).save(&deps.api.addr_canonicalize(&new_owner.as_str())?)?;

    Ok(Response::default())
}


pub fn try_update_validators_power(
    deps: DepsMut,
    info: MessageInfo,
    block_height: u64,
    validators: Vec<ValidatorWithPower>,
) -> StdResult<Response> {
    if deps.api.addr_canonicalize(&info.sender.as_str())? != owner(deps.storage).load()? {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    let mut total_validator_power_state = total_validator_power_read(deps.storage).load().unwrap();
    let mut validators_power_state = validators_power(deps.storage);
    for idx in 0usize..validators.len() {
        let validator = &validators[idx];
        let validator_power = match validators_power_state.get(validator.addr.as_slice()) {
            Some(data) => u128::from_str(String::from_utf8(data).unwrap().as_str()).unwrap(),
            None => 0u128,
        };
        total_validator_power_state = total_validator_power_state.sub(Uint128::from(validator_power));
        validators_power_state.set(validator.addr.as_slice(), validator.power.to_string().as_bytes());
        total_validator_power_state += validator.power;
    }
    total_validator_power(deps.storage).save(&total_validator_power_state)?;
    total_validator_power_last_updated(deps.storage).save(&block_height)?;

    Ok(Response::default())
}

fn verify_proof(
    root_hash: Vec<u8>,
    version: u64,
    key: Vec<u8>,
    data_hash: Vec<u8>,
    merkle_paths: Vec<iavl_merkle_path::Data>
) -> bool {
    let encoded_version: Vec<u8> = utils::encode_varint_signed(version);

    let mut hasher = Sha256::new();
    hasher.update([
        &[0u8],
        &[2u8],
        encoded_version.as_slice(),
        &[key.len() as u8],
        key.as_slice(),
        &[32u8],
        data_hash.as_slice(),
    ].concat());
    let mut current_merkle_hash = Vec::from(&hasher.finalize()[..]);

    for idx in 0usize..merkle_paths.len() {
        let merkle_path = merkle_paths[idx].clone();
        current_merkle_hash = merkle_path.get_parent_hash(&current_merkle_hash);
    }

    return current_merkle_hash == root_hash;
}

pub fn verify_oracle_data(
    oracle_state_root: Vec<u8>,
    result: result_codec::Result,
    version: u64,
    merkle_paths: Vec<iavl_merkle_path::Data>
) -> StdResult<VerifyOracleDataResponse> {
    let mut hasher = Sha256::new();
    hasher.update(result.clone().encode());
    let data_hash = &hasher.finalize()[..];
    let verify_proof = verify_proof(oracle_state_root, version, [&[255u8][..], &result.request_id.to_be_bytes()[..]].concat(), Vec::from(data_hash), merkle_paths);
    if !verify_proof {
        return Err(StdError::generic_err("INVALID_ORACLE_DATA_PROOF"));
    }
    Ok(VerifyOracleDataResponse { result })
}

pub fn try_relay_candidate_block(
    deps: DepsMut,
    info: MessageInfo,
    data: String,
) -> StdResult<Response> {
    let decoded_data: RelayCandidateBlockInput = OBIDecode::try_from_slice(HexDecode(data).unwrap().as_slice()).unwrap();
    let multi_store_decoded = decoded_data.multi_store;
    let merkle_paths_decoded = decoded_data.merkle_paths;
    let block_details_state_read = block_details_read(deps.storage);
    let total_validator_power_last_updated_state_read = total_validator_power_last_updated_read(deps.storage);
    let candidate_block_state_read = candidate_block_details_read(deps.storage);
    let block_height_data = merkle_paths_decoded.height;

    match &block_details_state_read.get(&block_height_data.to_be_bytes()) {
        Some(_data) => return Err(StdError::generic_err("Block height already relayed")),
        None => {},
    };

    let candidate_block_key = [info.sender.as_str().as_bytes(), &block_height_data.to_be_bytes()].concat();
    match &candidate_block_state_read.get(candidate_block_key.as_slice()) {
        Some(_data) => return Err(StdError::generic_err("Candidate block found for this sender and the specified block height [DUPLICATE]")),
        None => {},
    };

    match &total_validator_power_last_updated_state_read.load() {
        Ok(data) => {
            if data > &block_height_data {
                return Err(StdError::generic_err("Relayed data is already outdated"));
            }
        },
        Err(_e) => return Err(StdError::generic_err("Cannot load total validator power last updated state")),
    }

    let app_hash = multi_store_decoded.clone().get_app_hash();
    let block_header = merkle_paths_decoded.clone().get_block_header(app_hash.to_vec());
    let new_candidate_block_key = [info.sender.as_str().as_bytes(), &block_height_data.to_be_bytes()].concat();
    let new_candidate_block_detail = CandidateBlockDetail {
        block_header,
        last_signer_hex: String::from(""),
        sum_voting_power: 0u128,
        block_detail: BlockDetail {
            oracle_state: multi_store_decoded.oracle_iavl_state_hash.clone(),
            time_second: merkle_paths_decoded.time_second.clone(),
            time_nano_second_fraction: merkle_paths_decoded.time_nano_second.clone()
        }
    };
    let mut candidate_block_state = candidate_block_details(deps.storage);
    candidate_block_state.set(new_candidate_block_key.as_slice(), &bincode::serialize(&new_candidate_block_detail).unwrap());

    Ok(Response::default())
}

pub fn try_append_signature(
    deps: DepsMut,
    info: MessageInfo,
    data: String,
) -> StdResult<Response> {
    let decoded_data: AppendSignatureInput = OBIDecode::try_from_slice(HexDecode(data).unwrap().as_slice()).unwrap();
    let block_height = decoded_data.block_height;
    let signatures = decoded_data.signatures;
    let block_details_state_read = block_details_read(deps.storage);
    let total_validator_power_last_updated_state_read = total_validator_power_last_updated_read(deps.storage);
    let candidate_block_state_read = candidate_block_details_read(deps.storage);

    match &block_details_state_read.get(&block_height.to_be_bytes()) {
        Some(_data) => return Err(StdError::generic_err("Block height already relayed")),
        None => {},
    };

    match &total_validator_power_last_updated_state_read.load() {
        Ok(data) => {
            if data > &block_height {
                return Err(StdError::generic_err("Relayed data is already outdated"));
            }
        },
        Err(_e) => return Err(StdError::generic_err("Cannot load total validator power last updated state")),
    }

    let candidate_block_key = [info.sender.as_str().as_bytes(), &block_height.to_be_bytes()].concat();
    let mut candidate_block_detail: CandidateBlockDetail = match &candidate_block_state_read.get(candidate_block_key.as_slice()) {
        Some(data) => bincode::deserialize(data.as_slice()).unwrap(),
        None => return Err(StdError::generic_err("No candidate block found for this sender and the specified block height")),
    };

    let mut sum_voting_power = candidate_block_detail.clone().sum_voting_power;
    let mut last_signer_hex = candidate_block_detail.clone().last_signer_hex;
    let validators_power_state = validators_power_read(deps.storage);
    for idx in 0usize..signatures.len() {
        let signer = signatures[idx].clone().recover_signer(deps.as_ref(), &candidate_block_detail.block_header);
        if &HexEncode(signer.as_slice()).to_ascii_lowercase() <= &candidate_block_detail.last_signer_hex {
            return Err(StdError::generic_err("Invalid signature signer order"));
        }
        let value = match validators_power_state.get(signer.as_slice()) {
            Some(data) => u128::from_str(String::from_utf8(data).unwrap().as_str()).unwrap(),
            None => 0u128,
        };
        sum_voting_power += &value;
        last_signer_hex = HexEncode(signer.as_slice()).to_ascii_lowercase();
    }
    let total_validator_power_state = total_validator_power_read(deps.storage);
    if sum_voting_power * 3 <= total_validator_power_state.load().unwrap().u128() * 2 {
        let mut candidate_block_state = candidate_block_details(deps.storage);
        candidate_block_detail.last_signer_hex = last_signer_hex;
        candidate_block_detail.sum_voting_power = sum_voting_power;
        candidate_block_state.set(&candidate_block_key, &bincode::serialize(&candidate_block_detail).unwrap());
        let mut res = Response::default();
        res.attributes.push(Attribute { key: "Result".to_string(), value: "Signatures appended; Voting power is still too low".to_string() });
        return Ok(res);
    } else {
        let mut block_details_state = block_details(deps.storage);
        let new_block_detail = BlockDetail {
            oracle_state: candidate_block_detail.block_detail.oracle_state.clone(),
            time_second: candidate_block_detail.block_detail.time_second.clone(),
            time_nano_second_fraction: candidate_block_detail.block_detail.time_nano_second_fraction.clone(),
        };
        block_details_state.set(&block_height.to_be_bytes(), &bincode::serialize(&new_block_detail).unwrap());
        let mut res = Response::default();
        res.attributes.push(Attribute { key: "Result".to_string(), value: "Block detail relayed".to_string() });
        return Ok(res);
    }
}

pub fn try_verify_and_save_result(
    deps: DepsMut,
    data: String,
) -> StdResult<Response> {
    let decoded_data: VerifyAndSaveResultInput = OBIDecode::try_from_slice(HexDecode(data).unwrap().as_slice()).unwrap();

    let oracle_state_root = match &block_details_read(deps.storage).get(&decoded_data.block_height.to_be_bytes()) {
        Some(data) => {
            let block: BlockDetail = bincode::deserialize(data).unwrap();
            block.oracle_state
        },
        None => return Err(StdError::generic_err("NO_ORACLE_ROOT_STATE_DATA")),
    };

    let verify_result = match verify_oracle_data(oracle_state_root, decoded_data.result, decoded_data.version, decoded_data.merkle_paths) {
        Ok(result) => result.result,
        _ => return Err(StdError::generic_err("Failed to verify oracle data")),
    };
    let verified_result_key = &verify_result.request_id.to_be_bytes();
    let verified_result_serialized = bincode::serialize(&verify_result).unwrap();
    verified_results(deps.storage).set(verified_result_key, verified_result_serialized.as_slice());
    return Ok(Response::default());
}

pub fn try_remove_candidate_block(
    deps: DepsMut,
    info: MessageInfo,
    block_height: u64,
) -> StdResult<Response> {
    candidate_block_details(deps.storage).remove(&[info.sender.as_str().as_bytes(), &block_height.to_be_bytes()].concat());
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => to_binary(&try_query_owner(deps)?),
        QueryMsg::GetTotalVotingPower {} => to_binary(&try_query_total_validator_power(deps)?),
        QueryMsg::GetRecoverSigner { block_hash, signature_data } => to_binary(&try_query_recover_signer(deps, block_hash, signature_data)?),
        QueryMsg::GetVerifyOracleData { oracle_state_root, result, version, merkle_paths } => to_binary(&verify_oracle_data(oracle_state_root, result, version, merkle_paths)?),
        QueryMsg::GetValidatorPower { validator } => to_binary(&try_get_validator(deps, validator)?),
        QueryMsg::GetResult { request_id } => to_binary(&try_get_result(deps, request_id)?),
    }
}

pub fn try_query_owner(deps: Deps) -> StdResult<Addr> {
    owner_read(deps.storage)
        .load()
        .map(|addr| deps.api.addr_humanize(&addr).unwrap())
        .map_err(|_| StdError::generic_err("OWNER_NOT_INITIALIZED"))
}

pub fn try_query_total_validator_power(deps: Deps) -> StdResult<Uint128> {
    total_validator_power_read(deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("FAIL_TO_LOAD_TOTAL_VOTING_POWER"))
}

pub fn try_query_recover_signer(deps: Deps, block_hash: Vec<u8>, signature_data: tm_signature::Data) -> StdResult<CanonicalAddr> {
    Ok(signature_data.recover_signer(deps, &block_hash))
}

pub fn try_get_validator(
    deps: Deps,
    validator: CanonicalAddr,
) -> StdResult<GetValidatorPowerResponse> {
    let validators_power_state_read = validators_power_read(deps.storage);
    match validators_power_state_read.get(&validator.as_slice()) {
        Some(data) => Ok(GetValidatorPowerResponse { power: Uint128::from(u128::from_str(String::from_utf8(data).unwrap().as_str()).unwrap()) }),
        None => Err(StdError::not_found("Validator not found")),
    }
}

pub fn try_get_result(
    deps: Deps,
    request_id: u64,
) -> StdResult<result_codec::Result> {
    let verified_results_state_read = verified_results_read(deps.storage);
    match verified_results_state_read.get(&request_id.to_be_bytes()) {
        Some(data) => {
            let deserialized_result: result_codec::Result = bincode::deserialize(data.as_slice()).unwrap();
            Ok(deserialized_result)
        },
        None => Err(StdError::not_found("Verified result not found")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{CanonicalAddr, from_binary};
    use hex::decode;
/*
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let mut validators_set: Vec<ValidatorWithPower> = Vec::new();
        let a = ValidatorWithPower {
            addr: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()),
            power: Uint128::from(100u64),
        };
        validators_set.push(a);
        let msg = InitMsg { validators: validators_set };
        let env = mock_env("sender01", &[]);

        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn update_validator_power_test() {
        let mut deps = mock_dependencies(20, &[]);

        let validators_set: Vec<ValidatorWithPower> = vec![
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()),
                power: Uint128::from(100u64),
            }
        ];
        let msg = InitMsg { validators: validators_set };
        let env = mock_env("initiator", &[]);
        let _res = init(&mut deps, env, msg).unwrap();

        let msg = QueryMsg::GetValidatorPower { validator: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()) };
        let res: GetValidatorPowerResponse = from_binary(&query(&deps, msg).unwrap()).unwrap();
        assert_eq!(res, GetValidatorPowerResponse { power: Uint128::from(100u64) });

        let validators_set: Vec<ValidatorWithPower> = vec![
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()),
                power: Uint128::from(20u64),
            }
        ];
        let msg = ExecuteMsg::UpdateValidatorsPower { block_height: 3417u64, validators: validators_set };
        let env = mock_env("initiator", &[]);
        let _res = handle(&mut deps, env, msg).unwrap();

        let msg = QueryMsg::GetValidatorPower { validator: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()) };
        let res: GetValidatorPowerResponse = from_binary(&query(&deps, msg).unwrap()).unwrap();
        assert_eq!(res, GetValidatorPowerResponse { power: Uint128::from(20u64) });
    }

    #[test]
    fn verify_oracle_data_test() {
        let mut deps = mock_dependencies(20, &[]);
        let validators_set: Vec<ValidatorWithPower> = vec![
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("88e1cd00710495EEB93D4f522d16bC8B87Cb00FE").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("aAA22E077492CbaD414098EBD98AA8dc1C7AE8D9").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("B956589b6fC5523eeD0d9eEcfF06262Ce84ff260").unwrap()),
                power: Uint128::from(100u64),
            },
        ];
        let msg = InitMsg { validators: validators_set };
        let env = mock_env("sender01", &[]);
        let _res = init(&mut deps, env, msg).unwrap();

        let mock_block_detail = BlockDetail {
            oracle_state: decode("7920D562EC07A9979286FDCDA975F943D41D31974B01B8DC5B1B374878B194DA").unwrap(),
            time_second: 1622111198u64,
            time_nano_second_fraction: 1622111200u32,
        };

        block_details(&mut deps.storage).set(&3417u64.to_be_bytes(), &bincode::serialize(&mock_block_detail).unwrap());
        let query_data: BlockDetail = bincode::deserialize(&block_details_read(&deps.storage).get(&3417u64.to_be_bytes()).unwrap()).unwrap();
        assert_eq!(query_data, mock_block_detail);

        let result_data = result_codec::Result {
            client_id: String::from("from_scan"),
            oracle_script_id: 1u64,
            params: decode("0000000342544300000000000f4240").unwrap(),
            ask_count: 1u64,
            min_count: 1u64,
            request_id: 1u64,
            ans_count: 1u64,
            request_time: 1622111198u64,
            resolve_time: 1622111200u64,
            resolve_status: result_codec::ResolveStatus::ResolveStatusSuccess.to_u64(),
            result: decode("000000092b6826f2").unwrap(),
        };
        let merkle_paths_data = vec![
            iavl_merkle_path::Data {
                is_data_on_right: true,
                sub_tree_height: 1u8,
                sub_tree_size: 2u64,
                sub_tree_version: 1007u64,
                sibling_hash: decode("EB739BB22F48B7F3053A90BA2BA4FE07FAB262CADF8664489565C50FF505B8BD").unwrap(),
            },
            iavl_merkle_path::Data {
                is_data_on_right: true,
                sub_tree_height: 2u8,
                sub_tree_size: 4u64,
                sub_tree_version: 1007u64,
                sibling_hash: decode("BF32F8B214E4C36170D09B5125395C4EF1ABFA26583E676EF79AA3BA20A535A4").unwrap(),
            },
            iavl_merkle_path::Data {
                is_data_on_right: true,
                sub_tree_height: 3u8,
                sub_tree_size: 6u64,
                sub_tree_version: 1007u64,
                sibling_hash: decode("F732D5B5007633C64B77F6CCECF01ECAB2537501D28ED623B6EC97DA4C1C6005").unwrap(),
            },
            iavl_merkle_path::Data {
                is_data_on_right: true,
                sub_tree_height: 4u8,
                sub_tree_size: 10u64,
                sub_tree_version: 1007u64,
                sibling_hash: decode("F054C5E2412E1519951DBD7A60E2C5EDE41BABA494A6AF6FD0B0BAC4A4695C41").unwrap(),
            },
            iavl_merkle_path::Data {
                is_data_on_right: true,
                sub_tree_height: 5u8,
                sub_tree_size: 20u64,
                sub_tree_version: 3417u64,
                sibling_hash: decode("FFA5A376D4DCA03596020A9A256DF9B73FE42ADEF285DD0ABE7E89A9819144EF").unwrap(),
            },
        ];

        let res = try_verify_oracle_data(&deps, 3417u64, result_data.clone(), 1007u64, merkle_paths_data).unwrap();
        assert_eq!(res.result, result_data);
    }

    #[test]
    fn relay_candidate_block_test() {
        let mut deps = mock_dependencies(20, &[]);
        let validators_set: Vec<ValidatorWithPower> = vec![
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("88e1cd00710495EEB93D4f522d16bC8B87Cb00FE").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("aAA22E077492CbaD414098EBD98AA8dc1C7AE8D9").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("B956589b6fC5523eeD0d9eEcfF06262Ce84ff260").unwrap()),
                power: Uint128::from(100u64),
            },
        ];
        let msg = InitMsg { validators: validators_set };
        let env = mock_env("initiator", &[]);
        let _res = init(&mut deps, env, msg);

        let msg = ExecuteMsg::RelayCandidateBlock {
            data: "0000002039c31d3089788079e26877a8b51640be0fccd391833d45845db4d3d070adb1e40000002041f12e6f654e927f4476efe9c710aa5a3f1b4adf59e8899a34db578bdb1198d00000002099dad04860a6b4e777a6bfa60e75dc11cae48935556d570a7c625e5de52d4f8a00000020b18b165a5ca95fa274d3b79be5ca950146d837b8760c65563e58b1a4b4d4c93e00000020ca0a44d7055db7ef4dfce1aaec3599c1823945acaa1f91cae53239c07de00d6200000020b25be38e9445df8411de844c4980f1b452738bfc815bf71f49a378d3b00ff1c100000000000c9bef0000000060f68e6f1184adc6000000208fe8a8265123484f54f23797229d93550d2732e9f3bf1fc04a9b20f6b0b0bc1e00000020670fffc3a6123878ee2482ede280ff8a1f17e058e089cff0ccf8af0beb6709a700000020bbefff7e23a279218257ce0cf07edd7a1273f714943fc97e0edbec3f154de922000000200cbad0dd17b60213621a85d58b58231997c19e43d5d4a2d5cbe8a33cd5d6adc8".to_string(),
        };
        let env = mock_env("sender01", &[]);
        let res = handle(&mut deps, env, msg).unwrap();
        assert_eq!(res.messages.len(), 0);
    }

    #[test]
    fn append_signatures_test() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("initiator", &[]);
        let validators_set: Vec<ValidatorWithPower> = vec![
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("652D89a66Eb4eA55366c45b1f9ACfc8e2179E1c5").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("88e1cd00710495EEB93D4f522d16bC8B87Cb00FE").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("aAA22E077492CbaD414098EBD98AA8dc1C7AE8D9").unwrap()),
                power: Uint128::from(100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("B956589b6fC5523eeD0d9eEcfF06262Ce84ff260").unwrap()),
                power: Uint128::from(100u64),
            },
        ];
        let msg = InitMsg { validators: validators_set };
        let _res = init(&mut deps, env, msg);

        let msg = ExecuteMsg::RelayCandidateBlock {
            data: "0000002039c31d3089788079e26877a8b51640be0fccd391833d45845db4d3d070adb1e40000002041f12e6f654e927f4476efe9c710aa5a3f1b4adf59e8899a34db578bdb1198d00000002099dad04860a6b4e777a6bfa60e75dc11cae48935556d570a7c625e5de52d4f8a00000020b18b165a5ca95fa274d3b79be5ca950146d837b8760c65563e58b1a4b4d4c93e00000020ca0a44d7055db7ef4dfce1aaec3599c1823945acaa1f91cae53239c07de00d6200000020b25be38e9445df8411de844c4980f1b452738bfc815bf71f49a378d3b00ff1c100000000000c9bef0000000060f68e6f1184adc6000000208fe8a8265123484f54f23797229d93550d2732e9f3bf1fc04a9b20f6b0b0bc1e00000020670fffc3a6123878ee2482ede280ff8a1f17e058e089cff0ccf8af0beb6709a700000020bbefff7e23a279218257ce0cf07edd7a1273f714943fc97e0edbec3f154de922000000200cbad0dd17b60213621a85d58b58231997c19e43d5d4a2d5cbe8a33cd5d6adc8".to_string(),
        };
        let env = mock_env("sender02", &[]);
        let _res = handle(&mut deps, env, msg).unwrap();

        let env = mock_env("sender02", &[]);
        let msg = ExecuteMsg::AppendSignature {
            data: "00000000000c9bef00000005000000206f2b9c8c44f161a17325529a35ce2778865b6c69058b5bd83eb11a450f1d7e91000000202a3597fef73e65719a43d2dea64e1722472d12c857b2260e70b6b6bdc82e60821c0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610ad9af60f321362616e642d6c616f7a692d746573746e657432000000206bc382c99d0245a64dd45b7e0f8ea56cf58973dea77a91c25b5569d17841676f0000002065d1446fae6fde9f96c0d9241963b735858b79a859143c5163241e3f4db1cfa01b0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610fce5b511321362616e642d6c616f7a692d746573746e657432000000206d7cf8d300467a78b891342fe4f8df96d5f923063eb3b294e0e39ec1a5064fe40000002035a348554d22efc1eac9c0e9301ac1d2b70b09c008f45615866cf636fd1c07ff1c0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610a1c9f810321362616e642d6c616f7a692d746573746e657432000000202a89628ee70ef2b6207e8210d935118489f19572d9970c775f6e172a9619f5780000002023dbf8be4a23936a4ad08ab06f77bc300e887cb21a24f2382c40b9f7bd1d54a31b0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda8706109b8acc12321362616e642d6c616f7a692d746573746e6574320000002063265d4452c227568e87388a16a05d6938479a5689f46130075c558e35e986e600000020102530e354287e0ea1a9842057b4525bf086a558f7645736e577d72c3a2bbded1b0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610e9efe711321362616e642d6c616f7a692d746573746e657432".to_string(),
        };
        let _res = handle(&mut deps, env, msg).unwrap();
    }

    #[test]
    fn laozi_testnet_test() {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("david", &[]);
        let msg = InitMsg { validators: vec![
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("d01Cd301B2b20d1eB4b4a7DE3cF7Eb24d79F3dD4").unwrap()),
                power: Uint128::from(7100u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("f549422Dbc1372b75D73ea9B6316f45De8C773a8").unwrap()),
                power: Uint128::from(6455u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("A97Fb923C3C227D49eBE188a530ad43066313A9e").unwrap()),
                power: Uint128::from(101398u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("73f13A6e1E2b0F01FfAc734B4Ee4745DF325d911").unwrap()),
                power: Uint128::from(31099u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("0aaD682e5182AbF1C3C01f2Ee1FA1D69F33C7c02").unwrap()),
                power: Uint128::from(1034u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("37B1943787234904088174Bc279eE6bd9080C470").unwrap()),
                power: Uint128::from(1029u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("976fC977473fFA122957e0349E49Fa989Ce08D71").unwrap()),
                power: Uint128::from(1014u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("8B1B95e930bb7fa4ED1Cd641d0E1C447f6b53F37").unwrap()),
                power: Uint128::from(1013u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("5193c6eD280bc18c0821cA47921571706f47b74a").unwrap()),
                power: Uint128::from(101003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("384F9a9F1370595CB92011bbb512b83f373884F9").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("ECa89883dA4C8a4446cc0F1d0D2d6cdB307962B6").unwrap()),
                power: Uint128::from(101003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("8FF2EB3ef7a1eCD133304aC17cEF1b3e08e444f2").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("2Cc2aD43d1E8d701871436723dd2db6803Ae17c2").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("496b788De7e62C70aF792Cd323F52001b820D8Ce").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("b0304a3a6c75F41e1b156DadE6EC3dF7c7095610").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("0d94250Fe03d18f5dcc0E403A9b5D8C4D1190eA2").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("d23fA20dd0ee67aa1dbfdc8eaec1400215bd14b4").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("8bF0a2d02C8f9c1f273f77feAfB8485B4Aa45A7a").unwrap()),
                power: Uint128::from(1003u64),
            },
            ValidatorWithPower {
                addr: CanonicalAddr::from(decode("492fb1b9cd3d76F8851DfA46eDF206962857DA3B").unwrap()),
                power: Uint128::from(10u64),
            },
        ] };
        let _res = init(&mut deps, env, msg);

        let msg = ExecuteMsg::RelayCandidateBlock {
            data: "0000002039c31d3089788079e26877a8b51640be0fccd391833d45845db4d3d070adb1e40000002041f12e6f654e927f4476efe9c710aa5a3f1b4adf59e8899a34db578bdb1198d00000002099dad04860a6b4e777a6bfa60e75dc11cae48935556d570a7c625e5de52d4f8a00000020b18b165a5ca95fa274d3b79be5ca950146d837b8760c65563e58b1a4b4d4c93e00000020ca0a44d7055db7ef4dfce1aaec3599c1823945acaa1f91cae53239c07de00d6200000020b25be38e9445df8411de844c4980f1b452738bfc815bf71f49a378d3b00ff1c100000000000c9bef0000000060f68e6f1184adc6000000208fe8a8265123484f54f23797229d93550d2732e9f3bf1fc04a9b20f6b0b0bc1e00000020670fffc3a6123878ee2482ede280ff8a1f17e058e089cff0ccf8af0beb6709a700000020bbefff7e23a279218257ce0cf07edd7a1273f714943fc97e0edbec3f154de922000000200cbad0dd17b60213621a85d58b58231997c19e43d5d4a2d5cbe8a33cd5d6adc8".to_string(),
        };
        let env = mock_env("oatoat", &[]);
        let _res = handle(&mut deps, env, msg);

        let msg = ExecuteMsg::AppendSignature {
            data: "00000000000c9bef00000005000000206f2b9c8c44f161a17325529a35ce2778865b6c69058b5bd83eb11a450f1d7e91000000202a3597fef73e65719a43d2dea64e1722472d12c857b2260e70b6b6bdc82e60821c0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610ad9af60f321362616e642d6c616f7a692d746573746e657432000000206bc382c99d0245a64dd45b7e0f8ea56cf58973dea77a91c25b5569d17841676f0000002065d1446fae6fde9f96c0d9241963b735858b79a859143c5163241e3f4db1cfa01b0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610fce5b511321362616e642d6c616f7a692d746573746e657432000000206d7cf8d300467a78b891342fe4f8df96d5f923063eb3b294e0e39ec1a5064fe40000002035a348554d22efc1eac9c0e9301ac1d2b70b09c008f45615866cf636fd1c07ff1c0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610a1c9f810321362616e642d6c616f7a692d746573746e657432000000202a89628ee70ef2b6207e8210d935118489f19572d9970c775f6e172a9619f5780000002023dbf8be4a23936a4ad08ab06f77bc300e887cb21a24f2382c40b9f7bd1d54a31b0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda8706109b8acc12321362616e642d6c616f7a692d746573746e6574320000002063265d4452c227568e87388a16a05d6938479a5689f46130075c558e35e986e600000020102530e354287e0ea1a9842057b4525bf086a558f7645736e577d72c3a2bbded1b0000001077080211ef9b0c000000000022480a200000004812240801122064c28ed48945f64bbf5f7d641c89b0889fa7f9be88fa5074d90197d81a1c78522a0b08f29cda870610e9efe711321362616e642d6c616f7a692d746573746e657432".to_string(),
        };
        let env = mock_env("oatoat", &[]);
        let _res = handle(&mut deps, env, msg);

        let msg = ExecuteMsg::VerifyAndSaveResult {
            data: "00000000000c9bef0000000966726f6d5f7363616e000000000000002f00000014000000086e65775f7365656400000000000f4240000000000000000a000000000000000a0000000000080d26000000000000000a0000000060f505a50000000060f505b900000000000000010000004400000040d86016e9f39aeac6918ef72954448f6791a0b9ce2c156a6a485ce1fdd53b9a4eda20d2251eb30e2b9f6aa82c45e3460c1ccf9d4acd0b4e28fb34fbd4f9a1d24600000000000c1f94000000140101000000000000000200000000000c1f9400000020bd581c9039884c76f83c5b4cb8a0498635b95b1af6f35b13b4cc0cda11ad877d0102000000000000000300000000000c1f940000002044a4cab612a8e17ba549801051248d7aa59f1756b7b62fb6a8247e9fb029c9de0103000000000000000500000000000c1f9400000020629444f42963b8ab46fb6579f5f904c4c964b7d61a5608d9e91680ad020aecc40104000000000000000900000000000c1f94000000200dafb2ae6455293750b8fbd5d10ad7ff5630cd508b064a171eb5949a855cdb5f0105000000000000001900000000000c1f9b0000002097857481b07d60ca72a80a1de9d97d4848450b4b5341b6788c6d77c5a87da8c50106000000000000003800000000000c1fa500000020e6244ecb708d37ea1c916e1ef668fefab213c85b96816626830bfbde9c71cd860107000000000000007200000000000c1fbb00000020b04db6b3ffda68cb81afb9615e6ddc4be3751b5f802d84b874432ad7e8475cfc010800000000000000df00000000000c1fde00000020302b227f6ffd0a99be5e0774b7ffbb74d4171c8b9c82434d3910cff0fec16d4f010900000000000001c700000000000c2033000000206c80fe9448cec35b4e1444347674362bf511f25a4bd9954a9d425aff4999d739000a000000000000039900000000000c22250000002053fc8ce0aeb2cf7126408f0c34f999e01bf3ea456488bf99a69f3480c6b276c6000b000000000000074300000000000c261600000020a3b4a3726c9f69d3615ccc2a358dc662c7fbe31a4e4aaffca5ef5d98b29244a7010c0000000000000e8800000000000c28d2000000207a94916148bacf4e19ae36e6055d4e1a9e6c4a4f7601ad29e83a57d8dd74d419000d0000000000001d6700000000000c38b3000000204a6ea9c54a229e4ffb6535b02b67745b07f00159c0ea23e5334545a2e4a058c0000e0000000000003acd00000000000c57eb00000020a7e219b9c4684a0f61b9306485ac90700c707912467be1815149276148a72f21010f000000000000761900000000000c6cce000000208be3c670a74e7acff15c25684456cd38ef672607f04a6ca2482631d584e2acdf0110000000000000ebec00000000000c96e7000000208ac27ed9c31dc9291bd90a4278e0e52cb3711d91336b2a1c82292b76e1fab9140011000000000001675b00000000000c9bee000000201008eccf8008f6b3f648a05e6a546d4ca1294a1dd2817502a229a411eda3f88001120000000000033dad00000000000c9bee00000020939ec419b4857e138a26f8e3003e3190f94b63e0273a4ed119d258d39afd5fcc011300000000000513a200000000000c9bee00000020ecfcca113efcdcb23c504ef173643ea0db0576d4e41ad17b5903d6cbd2f117670114000000000008beb800000000000c9bee000000208078ca2f9045bd928571ac33eefd5fd1386129a8f450c657049534ddad7e476c".to_string(),
        };
        let env = mock_env("oatoat", &[]);
        let _res = handle(&mut deps, env, msg);

        let msg = QueryMsg::GetResult {
            request_id: 527654u64,
        };
        let res: result_codec::Result = from_binary(&query(&deps, msg).unwrap()).unwrap();
        println!("{:?}", res);
    }

 */
}
