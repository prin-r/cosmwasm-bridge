use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, Addr};
use crate::struct_types::ValidatorWithPower;
use crate::libraries::{iavl_merkle_path, result_codec, tm_signature};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub validators: Vec<ValidatorWithPower>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    TransferOwnership { new_owner: Addr },
    UpdateValidatorsPower { block_height: u64, validators: Vec<ValidatorWithPower> },
    RelayCandidateBlock { data: String },
    AppendSignature { data: String },
    VerifyAndSaveResult { data: String },
    RemoveCandidateBlock { block_height: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Owner {},
    GetTotalVotingPower {},
    GetRecoverSigner {
        block_hash: Vec<u8>,
        signature_data: tm_signature::Data,
    },
    GetVerifyOracleData {
        oracle_state_root: Vec<u8>,
        result: result_codec::Result,
        version: u64,
        merkle_paths: Vec<iavl_merkle_path::Data>
    },
    GetValidatorPower { validator: CanonicalAddr },
    GetResult { request_id: u64 },
}
