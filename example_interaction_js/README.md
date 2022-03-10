# Example contract interaction using bandchain.js and terra.js

This example show how to request yahoo's fantasy sport data from Band using `bandchain.js` and then submitted the data to a smart contract on Terra using `terra.js`.

### Example deployed contracts on Bombay-12

| Contract |      Address      |
|----------|:-------------:|
| Bridge   | [terra1l9drxzsmxrlspm73wurxnptsawyrn3s63k7qd4](https://finder.terra.money/testnet/address/terra1l9drxzsmxrlspm73wurxnptsawyrn3s63k7qd4) |
| Consumer | [terra1y6t238vszfgsgsztxmzger3lya8e22fknfjcrd](https://finder.terra.money/testnet/address/terra1y6t238vszfgsgsztxmzger3lya8e22fknfjcrd) |

### Steps

Please see [here](index.js#L193-L207)

1. Request data on Bandchain and then get its proof of existence
2. Relay the data with its proof to Band's bridge contract on Terra and then let the consumer contract to consume the data from Band's bridge contract
3. Try to read the latest saved result from the consumer contract

### Installation

- using node version v14.15.0
- yarn install

### Export env
```shell
export BAND_MNEMONIC="..."
export TERRA_MNEMONIC="..."
```

**Make sure that you have funded both accounts on Terra and Band.**

### Run

- node index.js

### Expected output

example output

```sh
=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
Submitting request to BandChain
txHash: 58F9F5D63494F6A56692AFE22B09D3B32CF5F74CFC4FD1ACC41069B281759DFD
Request ID: 2428361
{
  block_height: '4216242',
  oracle_data_proof: {
    result: {
      client_id: 'from_example_interaction_script',
      oracle_script_id: '134',
      calldata: 'AA==',
      ask_count: '4',
      min_count: '3',
      request_id: '2428361',
      ans_count: '3',
      request_time: '1646929742',
      resolve_time: '1646929749',
      resolve_status: 1,
      result: 'AAAA3lt7J2FkZHJlc3MnOiAndGVycmExd3UyNXV1dW14MG5jNmZrNGZjYXFrOG1jeDRkaG54d2NhNTZyYzUnLCAnc2NvcmUnOiA5N30sIHsnYWRkcmVzcyc6ICd0ZXJyYTFmbnZnc3M4dnY4ejg0dW1sYXYwZXkwc21oOTdncmtlZ3hsOXA1cicsICdzY29yZSc6IDgwfSwgeydhZGRyZXNzJzogJ3RlcnJhMWw5ZHJ4enNteHJsc3BtNzN3dXJ4bnB0c2F3eXJuM3M2M2s3cWQ0JywgJ3Njb3JlJzogODR9XQ=='
    },
    version: '4216241',
    merkle_paths: [
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object]
    ]
  },
  block_relay_proof: {
    multi_store_proof: {
      auth_to_fee_grant_stores_Merkle_hash: '8E4E61A76DB3F3096E05BD51516C85879E85F7C897DA572BAC411876352303C5',
      gov_to_ibc_core_stores_merkle_hash: 'B19FE73872969762F33466170053252FF0D1D465E16FADD2194A767BD0381197',
      mint_store_merkle_hash: '78DF288F2F7969887E3C1D0950E7EA3EBFCA612CB70FFBAEEAACF79D3A8A5687',
      oracle_iavl_State_hash: '9E59AE271A401AE5DFE6D0BF5EB11EF029116B67EDE6DC344D7A90124D150304',
      params_to_transfer_stores_merkle_hash: '7D34BB7BF5E5BE821DF81BBC831F48F634480F91150C3A30D760A17F7AEB9DB2',
      upgrade_store_merkle_hash: 'C9C8849ED125CC7681329C4D27B83B1FC8ACF7A865C9D1D1DF575CCA56F48DBE'
    },
    block_header_merkle_parts: {
      version_and_chain_id_hash: '4ED83077DFC5CDA908ED4C2F2A4246511C5DD48E1B480D59A6AC4C72A670A646',
      height: '4216242',
      time_second: '1646929753',
      time_nano_second: 777168707,
      last_block_id_and_other: '3B75A88A5FC8E0D7B5D3C3E28A3CE373F7AFBFA1F6E248893CE0A9EF6D55CF69',
      next_validator_hash_and_consensus_hash: 'CF2EFD4F60774153934A256788B66F27E7D318483EF7A53792DB87B73C0094AD',
      last_results_hash: '5494E2774AA081A42882671195CD6BE4240A832127E87604A2A67EFF2089B7E0',
      evidence_and_proposer_hash: '3AC24791F48DDEECEF095E3E41017B6E13609E9096550712F2EE20E381D5A16D'
    },
    signatures: [
      [Object], [Object],
      [Object], [Object],
      [Object], [Object],
      [Object], [Object],
      [Object], [Object],
      [Object], [Object],
      [Object], [Object]
    ]
  }
}
broadcast tx to terra chain:  1BE367A5E3042A648382F53C05DCAD00A6BAD0E64C564CA7B5BAFCA703E5CA57
polling: 1{
  code: 3,
  message: 'tx (1BE367A5E3042A648382F53C05DCAD00A6BAD0E64C564CA7B5BAFCA703E5CA57) not found: invalid request',
  details: []
}
polling: 2{
  code: 3,
  message: 'tx (1BE367A5E3042A648382F53C05DCAD00A6BAD0E64C564CA7B5BAFCA703E5CA57) not found: invalid request',
  details: []
}
polling: 3{
  code: 3,
  message: 'tx (1BE367A5E3042A648382F53C05DCAD00A6BAD0E64C564CA7B5BAFCA703E5CA57) not found: invalid request',
  details: []
}
polling: 4

latest saved result:  "[{'address': 'terra1wu25uuumx0nc6fk4fcaqk8mcx4dhnxwca56rc5', 'score': 97}, {'address': 'terra1fnvgss8vv8z84umlav0ey0smh97grkegxl9p5r', 'score': 80}, {'address': 'terra1l9drxzsmxrlspm73wurxnptsawyrn3s63k7qd4', 'score': 84}]"
=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
```
