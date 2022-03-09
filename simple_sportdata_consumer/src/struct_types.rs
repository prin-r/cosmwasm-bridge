use obi::{OBIDecode, OBIEncode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(OBIDecode, OBIEncode, Clone, Default, Debug, PartialEq)]
pub struct FinalResult {
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Result {
    pub client_id: String,
    pub oracle_script_id: u64,
    pub params: Vec<u8>,
    pub ask_count: u64,
    pub min_count: u64,
    pub request_id: u64,
    pub ans_count: u64,
    pub request_time: u64,
    pub resolve_time: u64,
    pub resolve_status: u64,
    pub result: Vec<u8>,
}

#[cfg(test)]
mod test {
    use super::*;
}