use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use hex::decode;
use sha2::{Sha256, Digest};
use obi::{OBIDecode, OBISchema, OBIEncode};

use crate::libraries::utils;

// MultiStoreProof stores a compact of other Cosmos-SDK modules' storage hash in multistore to
// compute (in combination with oracle store hash) Tendermint's application state hash at a given block.
//                                              ________________[AppHash]_________________
//                                             /                                          \
//                         _________________[I14]_________________                        [G]
//                        /                                        \
//             _______[I12]______                          _______[I13]________
//            /                  \                        /                    \
//       __[I8]__             __[I9]__                __[I10]__              __[I11]__
//      /         \          /         \            /          \            /         \
//    [I0]       [I1]     [I2]        [I3]        [I4]        [I5]        [I6]       [I7]
//   /   \      /   \    /    \      /    \      /    \      /    \      /    \     /    \
// [0]   [1]  [2]   [3] [4]   [5]  [6]    [7]  [8]    [9]  [A]    [B]  [C]    [D]  [E]   [F]
//
// [0] - auth     [1] - authz    [2] - bank    [3] - capability [4] - crisis  [5] - dist
// [6] - evidence [7] - feegrant [8] - gov     [9] - ibccore    [A] - mint    [B] - oracle
// [C] - params   [D] - slashing [E] - staking [F] - transfer   [G] - upgrade
//
// Notice that NOT all leaves of the Merkle tree are needed in order to compute the Merkle
// root hash, since we only want to validate the correctness of [B] In fact, only
// [A], [I4], [I11], [I12], and [G] are needed in order to compute [AppHash].

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, OBIDecode, OBISchema, OBIEncode)]
pub struct Data {
    pub auth_to_fee_grant_stores_merkle_hash: Vec<u8>, // [I12]
    pub gov_to_ibc_core_stores_merkle_hash: Vec<u8>, // [I4]
    pub mint_store_merkle_hash: Vec<u8>, // [A]
    pub oracle_iavl_state_hash: Vec<u8>, // [B]
    pub params_to_transfer_stores_merkle_hash: Vec<u8>, // [I11]
    pub upgrade_store_merkle_hash: Vec<u8>, // [G]
}

impl Data {
    pub fn get_app_hash(self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.oracle_iavl_state_hash);
        let hashed_oracle_iavl_state_hash = &hasher.finalize()[..];

        return utils::merkle_inner_hash( // [AppHash]
            utils::merkle_inner_hash( // [I14]
                self.auth_to_fee_grant_stores_merkle_hash, // [I12]
                utils::merkle_inner_hash( // [I13]
                    utils::merkle_inner_hash( // [I10]
                        self.gov_to_ibc_core_stores_merkle_hash, // [I4]
                        utils::merkle_inner_hash( // [I5]
                            self.mint_store_merkle_hash, // [A]
                            utils::merkle_leaf_hash( // [B]
                                [decode("066f7261636c6520").unwrap().as_slice(), hashed_oracle_iavl_state_hash].concat()
                            )
                        ),
                    ),
                    self.params_to_transfer_stores_merkle_hash // [I11]
                )
            ),
            self.upgrade_store_merkle_hash, // [G]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_app_hash_test() {
        // sample 1
        let data = Data {
            auth_to_fee_grant_stores_merkle_hash: decode("ACAA6420A7BB88830093A913CD39C304CACE64F2A40466CF8D08061D9B8F2485").unwrap(),
            gov_to_ibc_core_stores_merkle_hash: decode("3DF354440200A45608BA6A42C7243FB0289C9E8ACF9A13F1ED27759AD0EAF404").unwrap(),
            mint_store_merkle_hash: decode("06A9D989A4403F45DD2E053492260CC415C557009351850A607C8E7BAA17B0B7").unwrap(),
            oracle_iavl_state_hash: decode("CB45442287E8D3662215D6ED9C1E183CB5459DB06C8855464393A005427A37D5").unwrap(),
            params_to_transfer_stores_merkle_hash: decode("9A45D781D25741C42861701E1ACE5F198D4605451245C9ABC4A0E8F3D479340F").unwrap(),
            upgrade_store_merkle_hash: decode("C9C8849ED125CC7681329C4D27B83B1FC8ACF7A865C9D1D1DF575CCA56F48DBE").unwrap(),
        };
        let result = data.get_app_hash();
        assert_eq!(result, decode("0F4BF1298EDCEBC9AA2FF71BE9B3B70DD2200A12B903C94AB25A24CED2C67594").unwrap());

        // sample 2
        let data = Data {
            auth_to_fee_grant_stores_merkle_hash: decode("73FC154E0899059AFBC726D9BE23DE62739F198766503AEBBFA0119102E60831").unwrap(),
            gov_to_ibc_core_stores_merkle_hash: decode("FC61602EB36FE43248BEA691FF355059FE822404EC7846FB08ADE42283125B17").unwrap(),
            mint_store_merkle_hash: decode("9135F6A9A97C7B35D910A375F10DE1F3DBA7838C2DDAEAA01858D119D93A21B1").unwrap(),
            oracle_iavl_state_hash: decode("B5D3A6DD3EA412C4762495BCB8085F7A7E0FA1A1888EF60329732813C508AD90").unwrap(),
            params_to_transfer_stores_merkle_hash: decode("FEFECEF499539DC6F08AB7FD1ADB35B998C24ECCBD034BE4B03100FD54357513").unwrap(),
            upgrade_store_merkle_hash: decode("C9C8849ED125CC7681329C4D27B83B1FC8ACF7A865C9D1D1DF575CCA56F48DBE").unwrap(),
        };
        let result = data.get_app_hash();
        assert_eq!(result, decode("99A8E473585C107BD8080DFCB220A2905E930120D6A7E1BDBE7A3F18BA57D109").unwrap());
    }
}
