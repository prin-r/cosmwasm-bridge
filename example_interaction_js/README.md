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

### Run

- node index.js

### Expected output

example output

```sh
=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
Submitting request to BandChain
txHash: F0F626BB381550F98194312B4500283F3B87D5D39C46565476D4E06B87F74505
Request ID: 692183
{
  block_height: '931332',
  oracle_data_proof: {
    result: {
      client_id: 'from_yahoo_fantasy_example',
      oracle_script_id: '50',
      calldata: 'AAAAL2xlYWd1ZS8yMjMubC40MzEvcGxheWVycztwbGF5ZXJfa2V5cz0yMjMucC41NDc5AAAAHWxlYWd1ZSwxLHBsYXllcnMsMCxwbGF5ZXIsMCwy',
      ask_count: '10',
      min_count: '8',
      request_id: '692183',
      ans_count: '10',
      request_time: '1627036899',
      resolve_time: '1627036909',
      resolve_status: 1,
      result: 'AAAAcHsnbmFtZSc6IHsnYXNjaWlfZmlyc3QnOiAnRHJldycsICdhc2NpaV9sYXN0JzogJ0JyZWVzJywgJ2ZpcnN0JzogJ0RyZXcnLCAnZnVsbCc6ICdEcmV3IEJyZWVzJywgJ2xhc3QnOiAnQnJlZXMnfX0='
    },
    version: '931331',
    merkle_paths: [
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object], [Object],
      [Object], [Object]
    ]
  },
  block_relay_proof: {
    multi_store_proof: {
      auth_to_ibc_transfer_stores_Merkle_hash: 'B3FD5855F6DC54ABBF1D09233AAA7BBFC2B375942F81DA3900CFADC92CCB9A29',
      mint_store_merkle_hash: 'F588D9F0D8DF6E14973266EA66D954DC450CE9178740BDA1355610793164FEA9',
      oracle_iavl_State_hash: '0A298E02B77D5B1071705D1C705BAF4BB06A852074A30C7BA83C3029501CBFC4',
      params_to_slash_stores_merkle_hash: '57C77939CFE45E6D8B830745DE3E74C2FE189DEFA96EFAF66108F88C98566A85',
      staking_to_upgrade_stores_merkle_hash: 'FA8A461F575BFFE42C157C3966189FC4987BCF4C8D614D741BB36A0F5E311FF7'
    },
    block_header_merkle_parts: {
      version_and_chain_id_hash: 'B25BE38E9445DF8411DE844C4980F1B452738BFC815BF71F49A378D3B00FF1C1',
      height: '931332',
      time_second: '1627036911',
      time_nano_second: 761733185,
      last_block_id_and_other: 'CC1EE193E90FF182D1BA50DA1C2113FF65B5DC04C2C767D374B95BDB7FA859B3',
      next_validator_hash_and_consensus_hash: '4A511899C67C89604A654832B79FAD2B2DAA53B94CCBCE2C090CD9BF6CBFD4E2',
      last_results_hash: 'E3CD64D1E8A0657830DC6EB07DCAE1A17CA435FF1028811CA55C2CBCDE403E4A',
      evidence_and_proposer_hash: '0CBAD0DD17B60213621A85D58B58231997C19E43D5D4A2D5CBE8A33CD5D6ADC8'
    },
    signatures: [ [Object], [Object], [Object], [Object], [Object] ]
  }
}
broadcast tx to terra chain:  15B910AE07912B25EE39037AE6BE60B6C382EF57FFDE7A035F38A116E6C7D737
polling: 5

latest saved result:  "{'name': {'ascii_first': 'Drew', 'ascii_last': 'Brees', 'first': 'Drew', 'full': 'Drew Brees', 'last': 'Brees'}}"
=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
```