use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, Uint128};

use crate::libraries::result_codec::Result;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub validators: Vec<ValidatorWithPower>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    UpdateValidatorsPower { block_height: u64, validators: Vec<ValidatorWithPower> },
    RelayCandidateBlock { data: String },
    AppendSignature { data: String },
    VerifyAndSaveResult { data: String },
    RemoveCandidateBlock { block_height: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetValidatorPower { validator: CanonicalAddr },
    GetResult { request_id: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidatorWithPower {
    pub addr: CanonicalAddr,
    pub power: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VerifyOracleDataResponse {
    pub result: Result,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetValidatorPowerResponse {
    pub power: Uint128,
}
