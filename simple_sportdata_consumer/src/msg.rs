use cosmwasm_std::{Addr, CustomQuery};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub initial_bridge: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferOwnership { new_owner: Addr },
    SetBridge { new_bridge: Addr },
    SaveVerifiedResult { request_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Owner {},
    Bridge {},
    LatestSavedResult {},

    // Get result from bridge
    GetResult { request_id: u64 },
}

impl CustomQuery for QueryMsg {}