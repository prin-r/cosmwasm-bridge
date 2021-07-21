# Cosmwasm Bridge Contract
## Relay new block process
```
 msg:            RelayCandidateBlock
┌────────────────────────────────────────────────────┐
│                                                    │
│   Relay a candidate block with block relay proof   │
│            [multi store & block header]            │
│                                                    │
└─────────────────────────┬──────────────────────────┘
                          │
                          ▼
 msg:              AppendSignature
┌─────────────────────────────────────────────────────┐
│                                                     │
│  Append block signatures to the provided candidate  │
│                                                     │
└─────────────────────────────────────────────────────┘
```
## Contract Handle Messages
### `UpdateValidatorsPower`
Update the voting power of provided validators. Return error if message sender is not the contract owner.
```json
{
  "update_validators_power": {
    "validators": [
      {
        "addr": <base64-encoded address>,
        "power": <new voting power>,
      },
      ...
    ]
  }
}
```

### `RelayCandidateBlock`
Relay a candidate block to the contract. A candidate block will get relayed if the total voting power of its appended signature exceeds two-thirds of the total voting power of all validators. Each candidate block is referenced by its block height and the message sender.
```json
{
  "relay_candidate_block": {
    "data": <string>
  }
}
```
Where data string is the OBI-encoded data in the following format:
```
{multi_store:{auth_to_ibc_transfer_stores_merkle_hash:[u8],mint_store_merkle_hash:[u8],oracle_iavl_state_hash:[u8],params_to_slash_stores_merkle_hash:[u8],staking_to_upgrade_stores_merkle_hash:[u8]},merkle_paths:{version_and_chain_id_hash:[u8],height:u64,time_second:u64,time_nano_second:u32,last_block_id_and_other:[u8],next_validator_hash_and_consensus_hash:[u8],last_results_hash:[u8],evidence_and_proposer_hash:[u8]}}/{_:u8}
```

### `AppendSignature`
Append the block signatures to the specified candidate block.
```json
{
  "append_signature": {
    "data": <string>
  }
}
```
Where data string is the OBI-encoded data in the following format:
```
{block_height:u64,signatures:[{r:[u8],s:[u8],v:u8,signed_data_prefix:[u8],signed_data_suffix:[u8]}]}/{_:u8}
```

### `VerifyAndSaveResult`
Verify the provided result with the relayed block information in the contract state. Save the verified result to the contract state.
```json
{
  "verify_and_save_result": {
    "data": <string>
  }
}
```
Where data string is the OBI-encoded data in the following format:
```
{block_height:u64,result:{client_id:string,oracle_script_id:u64,calldata:[u8],ask_count:u64,min_count:u64,request_id:u64,ans_count:u64,request_time:u64,resolve_time:u64,resolve_status:u64,result:[u8]},version:u64,merkle_paths:[{is_data_on_right:u8,subtree_height:u8,subtree_size:u64,subtree_version:u64,sibling_hash:[u8]}]}/{_:u8}
```

### `RemoveCandidateBlock`
Remove a candidate block from the contract state. Return error if the message sender is not the owner of the specified candidate block.
```json
{
  "remove_candidate_block": {
    "block_height": <block-height>
  }
}
```

## Contract Query Messages
### `GetValidatorPower`
Get the voting power of the provided validator.
```json
{
  "get_validator_power": {
    "validator": <base64-encoded address>
  }
}
```

### `GetResult`
Get the verified result of the specified request id.
```json
{
  "get_result": {
    "request_id": <request-id>
  }
}
```
