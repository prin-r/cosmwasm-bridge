use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128, CanonicalAddr};
use crate::libraries::result_codec::Result;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BlockDetail {
    pub oracle_state: Vec<u8>,
    pub time_second: u64,
    pub time_nano_second_fraction: u32, // between 0 to 10^9
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CandidateBlockDetail {
    pub block_header: Vec<u8>,
    pub last_signer_hex: String,
    pub sum_voting_power: u128,
    pub block_detail: BlockDetail,
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
