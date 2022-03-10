use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{bridge, bridge_read, owner, owner_read, result, result_read};
use crate::struct_types::{FinalResult, Result};
use cosmwasm_std::{
    to_binary, Binary, Env, StdError, Addr, StdResult, WasmQuery, entry_point, DepsMut, MessageInfo,
    Response, Deps, Attribute
};
use obi::OBIDecode;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    owner(deps.storage).save(&deps.api.addr_canonicalize(&info.sender.as_str())?)?;
    bridge(deps.storage).save(&deps.api.addr_canonicalize(&msg.initial_bridge.as_str())?)?;
    let mut res = Response::default();
    res.attributes.push(Attribute { key: "Owner".to_string(), value: info.sender.to_string() });
    res.attributes.push(Attribute { key: "Bridge".to_string(), value: msg.initial_bridge.to_string() });
    Ok(res)
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
        ExecuteMsg::SetBridge { new_bridge } => try_set_bridge(deps, info, new_bridge),
        ExecuteMsg::SaveVerifiedResult { request_id } => try_verify_and_save(deps, info, request_id),
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

    let mut res = Response::default();
    res.attributes.push(Attribute { key: "NewOwner".to_string(), value: new_owner.to_string() });

    Ok(res)
}

pub fn try_set_bridge(
    deps: DepsMut,
    info: MessageInfo,
    new_bridge: Addr,
) -> StdResult<Response> {
    if deps.api.addr_canonicalize(&info.sender.as_str())? != owner(deps.storage).load()? {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    bridge(deps.storage).save(&deps.api.addr_canonicalize(&new_bridge.as_str())?)?;

    let mut res = Response::default();
    res.attributes.push(Attribute { key: "NewBridge".to_string(), value: new_bridge.to_string() });

    Ok(res)
}

pub fn try_verify_and_save(
    deps: DepsMut,
    _info: MessageInfo,
    request_id: u64,
) -> StdResult<Response> {
    let verified_result = &query_latest_verified_result_by_request_id(deps.as_ref(), request_id)?;

    if verified_result.resolve_status != 1 {
        return Err(StdError::generic_err(
            "FAIL_REQUEST_IS_NOT_SUCCESSFULLY_RESOLVED",
        ));
    }

    let final_result = FinalResult::try_from_slice(&verified_result.result)
        .map_err(|_| StdError::generic_err("FAIL_TO_PARSE_FINAL_RESULT"))?;

    result(deps.storage).save(&final_result.value)?;

    let mut res = Response::default();
    res.attributes.push(Attribute { key: "ConsumeResult".to_string(), value: final_result.value.to_string() });

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => to_binary(&try_query_owner(deps)?),
        QueryMsg::Bridge {} => to_binary(&try_query_bridge(deps)?),
        QueryMsg::GetResult { request_id } => to_binary(&query_latest_verified_result_by_request_id(deps, request_id)?),
        QueryMsg::LatestSavedResult {} => to_binary(&query_latest_saved_result(deps)?),
    }
}

pub fn try_query_owner(deps: Deps) -> StdResult<Addr> {
    owner_read(deps.storage)
        .load()
        .map(|addr| deps.api.addr_humanize(&addr).unwrap())
        .map_err(|_| StdError::generic_err("OWNER_NOT_INITIALIZED"))
}

pub fn try_query_bridge(deps: Deps) -> StdResult<Addr> {
    bridge_read(deps.storage)
        .load()
        .map(|addr| deps.api.addr_humanize(&addr).unwrap())
        .map_err(|_| StdError::generic_err("BRIDGE_NOT_INITIALIZED"))
}

fn query_latest_verified_result_by_request_id(deps: Deps, request_id: u64) -> StdResult<Result> {
    Ok(deps.querier.custom_query::<QueryMsg, Result>(
        &WasmQuery::Smart {
            contract_addr: (try_query_bridge(deps)?).to_string(),
            msg: to_binary(&QueryMsg::GetResult { request_id })?,
        }
        .into(),
    )?)
}

fn query_latest_saved_result(deps: Deps) -> StdResult<String> {
    result_read(deps.storage)
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