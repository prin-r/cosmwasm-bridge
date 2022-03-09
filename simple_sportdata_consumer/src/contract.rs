use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{bridge, bridge_read, owner, owner_read, result, result_read};
use crate::struct_types::{FinalResult, Result};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, WasmQuery,
};
use cosmwasm_std::{CanonicalAddr, HumanAddr};
use obi::{OBIDecode, OBIEncode};

macro_rules! unwrap_query {
    ( $e:expr, $f:expr ) => {
        match $e {
            Ok(x) => match to_binary(&x) {
                Ok(y) => Ok(y),
                Err(_) => Err(StdError::generic_err($f)),
            },
            Err(e) => return Err(e),
        }
    };
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    owner(&mut deps.storage).save(&deps.api.canonical_address(&env.message.sender)?)?;
    bridge(&mut deps.storage).save(&deps.api.canonical_address(&msg.initial_bridge)?)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::TransferOwnership { new_owner } => try_transfer_ownership(deps, env, new_owner),
        HandleMsg::SetBridge { new_bridge } => try_set_bridge(deps, env, new_bridge),
        HandleMsg::SaveVerifiedResult { request_id } => try_verify_and_save(deps, env, request_id),
    }
}

pub fn try_transfer_ownership<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new_owner: HumanAddr,
) -> StdResult<HandleResponse> {
    let owner_addr = owner(&mut deps.storage).load()?;
    if deps.api.canonical_address(&env.message.sender)? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    owner(&mut deps.storage).save(&deps.api.canonical_address(&new_owner)?)?;

    Ok(HandleResponse::default())
}

pub fn try_set_bridge<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new_bridge: HumanAddr,
) -> StdResult<HandleResponse> {
    let owner_addr = owner(&mut deps.storage).load()?;
    if deps.api.canonical_address(&env.message.sender)? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    bridge(&mut deps.storage).save(&deps.api.canonical_address(&new_bridge)?)?;

    Ok(HandleResponse::default())
}

pub fn try_verify_and_save<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    request_id: u64,
) -> StdResult<HandleResponse> {
    let verified_result = &query_latest_verified_result_by_request_id(deps, request_id)?;

    if verified_result.resolve_status != 1 {
        return Err(StdError::generic_err(
            "FAIL_REQUEST_IS_NOT_SUCCESSFULLY_RESOLVED",
        ));
    }

    let final_result = FinalResult::try_from_slice(&verified_result.result)
        .map_err(|_| StdError::generic_err("FAIL_TO_PARSE_FINAL_RESULT"))?;

    result(&mut deps.storage).save(&final_result.value)?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => unwrap_query!(query_owner(deps), "SERIALIZE_OWNER_ERROR"),
        QueryMsg::Bridge {} => unwrap_query!(query_bridge(deps), "SERIALIZE_BRIDGE_DATA_ERROR"),
        QueryMsg::GetResult { request_id } => unwrap_query!(
            query_latest_verified_result_by_request_id(deps, request_id),
            "SERIALIZE_VERIFIED_RESULT_ERROR"
        ),
        QueryMsg::LatestSavedResult {} => unwrap_query!(
            query_latest_saved_result(deps),
            "SERIALIZE_LATEST_SAVED_RESULT_ERROR"
        ),
    }
}

fn query_owner<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CanonicalAddr> {
    owner_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("OWNER_NOT_INITIALIZED"))
}

fn query_bridge<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<CanonicalAddr> {
    bridge_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("BRIDGE_NOT_INITIALIZED"))
}

fn query_latest_verified_result_by_request_id<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    request_id: u64,
) -> StdResult<Result> {
    deps.querier.custom_query::<QueryMsg, Result>(
        &WasmQuery::Smart {
            contract_addr: deps.api.human_address(&query_bridge(deps)?)?,
            msg: to_binary(&QueryMsg::GetResult { request_id })?,
        }
        .into(),
    )
}

fn query_latest_saved_result<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<String> {
    result_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("LATEST_SAVED_RESULT_NOT_INITIALIZED"))
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn test_1() {
        let val:Vec<u8> = vec![0,0,0,0,0,0,0,3,4,2,5,4,4,3];
        let final_result = FinalResult::try_from_slice(&val);
        println!("{:?}", final_result);

        let x:FinalResult = FinalResult { value: "BTC".into() };
        println!("{:?}", x);
        println!("{:?}", x.try_to_vec());
    }
    å
     */
}