use obi::{OBIDecode, OBISchema, OBIEncode};

use crate::libraries::multi_store;
use crate::libraries::block_header_merkle_path;
use crate::libraries::tm_signature;
use crate::libraries::result_codec;
use crate::libraries::iavl_merkle_path;

#[derive(OBIDecode, OBISchema, OBIEncode, Debug)]
pub struct RelayCandidateBlockInput {
    pub multi_store: multi_store::Data,
    pub merkle_paths: block_header_merkle_path::Data,
}

#[derive(OBIDecode, OBISchema, OBIEncode, Debug)]
pub struct AppendSignatureInput {
    pub block_height: u64,
    pub signatures: Vec<tm_signature::Data>,
}

#[derive(OBIDecode, OBISchema, OBIEncode, Debug)]
pub struct VerifyAndSaveResultInput {
    pub block_height: u64,
    pub result: result_codec::Result,
    pub version: u64,
    pub merkle_paths: Vec<iavl_merkle_path::Data>
}

#[cfg(test)]
mod test {
    use super::*;
    use base64::decode as b64decode;
    use hex::{encode as HexEncode, decode as HexDecode};

    #[test]
    fn decode_test() {
        // let res: RelayCandidateBlockInput = OBIDecode::try_from_slice(HexDecode("00000040333943333144333038393738383037394532363837374138423531363430424530464343443339313833334434353834354442344433443037304144423145340000004034314631324536463635344539323746343437364546453943373130414135413346314234414446353945383839394133344442353738424442313139384430000000403939444144303438363041364234453737374136424641363045373544433131434145343839333535353644353730413743363235453544453532443446384100000040423138423136354135434139354641323734443342373942453543413935303134364438333742383736304336353536334535384231413442344434433933450000004043413041343444373035354442374546344446434531414145433335393943313832333934354143414131463931434145353332333943303744453030443632000000404232354245333845393434354446383431314445383434433439383046314234353237333842464338313542463731463439413337384433423030464631433100000000000c9bef0000000060f68e6f1184adc60000004038464538413832363531323334383446353446323337393732323944393335353044323733324539463342463146433034413942323046364230423042433145000000403637304646464333413631323338373845453234383245444532383046463841314631374530353845303839434646304343463841463042454236373039413700000040424245464646374532334132373932313832353743453043463037454444374131323733463731343934334643393745304544424543334631353444453932320000004030434241443044443137423630323133363231413835443538423538323331393937433139453433443544344132443543424538413333434435443641444338").unwrap().as_slice()).unwrap();
        // println!("{:?}", res);
        let res: AppendSignatureInput = OBIDecode::try_from_slice(HexDecode("00000000000c9bef00000005000000403646324239433843343446313631413137333235353239413335434532373738383635423643363930353842354244383345423131413435304631443745393100000040324133353937464546373345363537313941343344324445413634453137323234373244313243383537423232363045373042364236424443383245363038321c00000020373730383032313145463942304330303030303030303030323234383041323000000090313232343038303131323230363443323845443438393435463634424246354637443634314338394230383839464137463942453838464135303734443930313937443831413143373835323241304230384632394344413837303631304144394146363046333231333632363136453634324436433631364637413639324437343635373337343645363537343332000000403642433338324339394430323435413634444434354237453046384541353643463538393733444541373741393143323542353536394431373834313637364600000040363544313434364641453646444539463936433044393234313936334237333538353842373941383539313433433531363332343145334634444231434641301b00000020373730383032313145463942304330303030303030303030323234383041323000000090313232343038303131323230363443323845443438393435463634424246354637443634314338394230383839464137463942453838464135303734443930313937443831413143373835323241304230384632394344413837303631304643453542353131333231333632363136453634324436433631364637413639324437343635373337343645363537343332000000403644374346384433303034363741373842383931333432464534463844463936443546393233303633454233423239344530453339454331413530363446453400000040333541333438353534443232454643314541433943304539333031414331443242373042303943303038463435363135383636434636333646443143303746461c00000020373730383032313145463942304330303030303030303030323234383041323000000090313232343038303131323230363443323845443438393435463634424246354637443634314338394230383839464137463942453838464135303734443930313937443831413143373835323241304230384632394344413837303631304131433946383130333231333632363136453634324436433631364637413639324437343635373337343645363537343332000000403241383936323845453730454632423632303745383231304439333531313834383946313935373244393937304337373546364531373241393631394635373800000040323344424638424534413233393336413441443038414230364637374243333030453838374342323141323446323338324334304239463742443144353441331b00000020373730383032313145463942304330303030303030303030323234383041323000000090313232343038303131323230363443323845443438393435463634424246354637443634314338394230383839464137463942453838464135303734443930313937443831413143373835323241304230384632394344413837303631303942384143433132333231333632363136453634324436433631364637413639324437343635373337343645363537343332000000403633323635443434353243323237353638453837333838413136413035443639333834373941353638394634363133303037354335353845333545393836453600000040313032353330453335343238374530454131413938343230353742343532354246303836413535384637363435373336453537374437324333413242424445441b00000020373730383032313145463942304330303030303030303030323234383041323000000090313232343038303131323230363443323845443438393435463634424246354637443634314338394230383839464137463942453838464135303734443930313937443831413143373835323241304230384632394344413837303631304539454645373131333231333632363136453634324436433631364637413639324437343635373337343645363537343332").unwrap().as_slice()).unwrap();
        println!("{:?}", res);
        let res: VerifyAndSaveResultInput = OBIDecode::try_from_slice(HexDecode("00000000000c9bef0000000966726f6d5f7363616e000000000000002f00000014000000086e65775f7365656400000000000f4240000000000000000a000000000000000a0000000000080d26000000000000000a0000000060f505a50000000060f505b900000000000000010000004400000040d86016e9f39aeac6918ef72954448f6791a0b9ce2c156a6a485ce1fdd53b9a4eda20d2251eb30e2b9f6aa82c45e3460c1ccf9d4acd0b4e28fb34fbd4f9a1d24600000000000c1f94000000140101000000000000000200000000000c1f9400000020bd581c9039884c76f83c5b4cb8a0498635b95b1af6f35b13b4cc0cda11ad877d0102000000000000000300000000000c1f940000002044a4cab612a8e17ba549801051248d7aa59f1756b7b62fb6a8247e9fb029c9de0103000000000000000500000000000c1f9400000020629444f42963b8ab46fb6579f5f904c4c964b7d61a5608d9e91680ad020aecc40104000000000000000900000000000c1f94000000200dafb2ae6455293750b8fbd5d10ad7ff5630cd508b064a171eb5949a855cdb5f0105000000000000001900000000000c1f9b0000002097857481b07d60ca72a80a1de9d97d4848450b4b5341b6788c6d77c5a87da8c50106000000000000003800000000000c1fa500000020e6244ecb708d37ea1c916e1ef668fefab213c85b96816626830bfbde9c71cd860107000000000000007200000000000c1fbb00000020b04db6b3ffda68cb81afb9615e6ddc4be3751b5f802d84b874432ad7e8475cfc010800000000000000df00000000000c1fde00000020302b227f6ffd0a99be5e0774b7ffbb74d4171c8b9c82434d3910cff0fec16d4f010900000000000001c700000000000c2033000000206c80fe9448cec35b4e1444347674362bf511f25a4bd9954a9d425aff4999d739000a000000000000039900000000000c22250000002053fc8ce0aeb2cf7126408f0c34f999e01bf3ea456488bf99a69f3480c6b276c6000b000000000000074300000000000c261600000020a3b4a3726c9f69d3615ccc2a358dc662c7fbe31a4e4aaffca5ef5d98b29244a7010c0000000000000e8800000000000c28d2000000207a94916148bacf4e19ae36e6055d4e1a9e6c4a4f7601ad29e83a57d8dd74d419000d0000000000001d6700000000000c38b3000000204a6ea9c54a229e4ffb6535b02b67745b07f00159c0ea23e5334545a2e4a058c0000e0000000000003acd00000000000c57eb00000020a7e219b9c4684a0f61b9306485ac90700c707912467be1815149276148a72f21010f000000000000761900000000000c6cce000000208be3c670a74e7acff15c25684456cd38ef672607f04a6ca2482631d584e2acdf0110000000000000ebec00000000000c96e7000000208ac27ed9c31dc9291bd90a4278e0e52cb3711d91336b2a1c82292b76e1fab9140011000000000001675b00000000000c9bee000000201008eccf8008f6b3f648a05e6a546d4ca1294a1dd2817502a229a411eda3f88001120000000000033dad00000000000c9bee00000020939ec419b4857e138a26f8e3003e3190f94b63e0273a4ed119d258d39afd5fcc011300000000000513a200000000000c9bee00000020ecfcca113efcdcb23c504ef173643ea0db0576d4e41ad17b5903d6cbd2f117670114000000000008beb800000000000c9bee000000208078ca2f9045bd928571ac33eefd5fd1386129a8f450c657049534ddad7e476c").unwrap().as_slice()).unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_1() {
        let candidate_block_input = RelayCandidateBlockInput {
            multi_store: multi_store::Data {
                auth_to_fee_grant_stores_merkle_hash: HexDecode("73FC154E0899059AFBC726D9BE23DE62739F198766503AEBBFA0119102E60831").unwrap(),
                gov_to_ibc_core_stores_merkle_hash: HexDecode("FC61602EB36FE43248BEA691FF355059FE822404EC7846FB08ADE42283125B17").unwrap(),
                mint_store_merkle_hash: HexDecode("9135F6A9A97C7B35D910A375F10DE1F3DBA7838C2DDAEAA01858D119D93A21B1").unwrap(),
                oracle_iavl_state_hash: HexDecode("B5D3A6DD3EA412C4762495BCB8085F7A7E0FA1A1888EF60329732813C508AD90").unwrap(),
                params_to_transfer_stores_merkle_hash: HexDecode("FEFECEF499539DC6F08AB7FD1ADB35B998C24ECCBD034BE4B03100FD54357513").unwrap(),
                upgrade_store_merkle_hash: HexDecode("C9C8849ED125CC7681329C4D27B83B1FC8ACF7A865C9D1D1DF575CCA56F48DBE").unwrap(),
            },
            merkle_paths: block_header_merkle_path::Data {
                version_and_chain_id_hash: HexDecode("4ED83077DFC5CDA908ED4C2F2A4246511C5DD48E1B480D59A6AC4C72A670A646").unwrap(),
                height: 3990172,
                time_second: 1646047506,
                time_nano_second: 751892973,
                last_block_id_and_other: HexDecode("74D8A68CDD98D615A19334FE110064789DC440CDD22FA75EA64B6667F3806B56").unwrap(),
                next_validator_hash_and_consensus_hash: HexDecode("7183BA4364C7944ACE898286480DF1BF82F4E9B75861173D3EA51FEF42A2220D").unwrap(),
                last_results_hash: HexDecode("12839198C0BCF52763DCD4623487D37AEC4040729E7B9E0B79816FBAADEFBB52").unwrap(),
                evidence_and_proposer_hash: HexDecode("C5BC10860C2308B637427A7E20B8847E42D187E6381B638CFA9C9AA4FF9BBCC2").unwrap(),
            }
        };
        println!("RelayCandidateBlockInput: {:?}", HexEncode(OBIEncode::try_to_vec(&candidate_block_input).unwrap()));
    }

    #[test]
    fn test_2() {
        let append_signature_input = AppendSignatureInput {
            block_height: 3990172,
            signatures: vec![
                tm_signature::Data {
                    r:HexDecode("24865F507AC4F8DA01AC827B8F6C86649660AD827CD5FDAC3F2FD55491AF1FC7").unwrap(),
                    s:HexDecode("3FCB8DC19C2601C88E01423DED59EBE1397D5637A83CF2720B4617EF42C92741").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F290061085D38112321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("0D0F38AE8998C43A69A775E45BFE63A8749C770B47663D9F77143E0B2C05455B").unwrap(),
                    s:HexDecode("6551B13F829CCCFDDEDFA9268344D022F086965F511CCC21D63F4366305ACA6B").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610E7E68714321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("6A589678A55BB70FB66934AA1FB35BE31947BB836D5732AD81E7DFC97D4AFCA6").unwrap(),
                    s:HexDecode("580C01B4814756E0FA1B2A02E4F805E09F6F7560142532B007302F82EF383C59").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610EFCDDB1A321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("CFB2694873302C7179CA1F479CEA45694E9893B991DC40483FE5606CAA5F6FFC").unwrap(),
                    s:HexDecode("1DDB14A699925FE305B9E914725D63AEE63C8F5D150F1217F8DB9FF74A8EEC50").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610E2F9B81B321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("D610A9818E27F4FF0FCD8DDF15BFF1222A7EBB67FE14D557856BBAA28F7DE89A").unwrap(),
                    s:HexDecode("3A1720F60B2757A39A5165C19BE15D90D99C44D33C3D46CB89472C1925B1CB7F").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F290061090F7C01A321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("A3413B4BB7FC838BBE2E536D77C34D3B2E3A9E827CBEDFA3E3F861A7B7009451").unwrap(),
                    s:HexDecode("6DBC2EA4532E6F53089E0978DEE10FFB4AE7D6D9AC49F5D890D6D8C89299AF04").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610D9A2D713321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("71E6B80DF2EC023CCA08589E3D32CAD6654958D871532C16DDFD9C6E6C521CA4").unwrap(),
                    s:HexDecode("25F46E3292BE55E515927EEA4806B581DAD2B5FC5D3635056FA2788B791814FF").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610F3E79B08321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("89D32DDFADE1CC40F47276E484F7E1E7D7B3C385B8D9A7AB439AA111AD4DC20B").unwrap(),
                    s:HexDecode("28AB566C3AF0360C06CB750CBB5E1CDDE1B51BE59F0884386363FFA42075AE50").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("780802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0C0894E2F2900610B6AEECDA03321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("6206F14C2D5ECD013415909531A3675383BD4B868F279605B2519E09D2F0B368").unwrap(),
                    s:HexDecode("1D104F5C5B6A5A8F6A08365207365A6E8835AE7157CFF2F707247250DAFA1935").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610E5E3BD1B321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("EB7ADF7023C77266A4C701A05474D4482EF4835A48EC11B86D92F756BBCF11A8").unwrap(),
                    s:HexDecode("033E51B0042530F2C69C29A322B7794680D044D51102F8175DA64B7F72BD8DFD").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("780802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0C0894E2F2900610E1D0D1D303321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("1EA626CA1602FADD1C37D93E9628FAD858FCA1733672FB17A9B78177FB3FF32A").unwrap(),
                    s:HexDecode("6649076CC5E029D2F1045C36B4DC071CC6E0AE2C2C046A717510C133E82C5384").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610D3CFF906321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("CB1718700E4D652ACFD7CFCFFF7175BE8ED97AE1F0FAB3F69510A61FD4B2F31E").unwrap(),
                    s:HexDecode("124C1967878B4898314DB8857BC64176BAC71D67613AD45E2566CE052BACE237").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F290061082C4D01C321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("04A19CCD67AF741FBCF0092F05E6E7859A66350D197A49BFF816DC9BF29D0D5D").unwrap(),
                    s:HexDecode("6ECA863691D37D84F75D25F4E0CC5A6EB5B41B92104F0CC6FD07EF89754803C0").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610FBF3B71E321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("75DD2AED4F4E35391851C6CAD30DA509BF15F8839B1C086A3C48EA8F57D8D001").unwrap(),
                    s:HexDecode("4D161FEDB8EC60E320B5C96E5998EBD85961019FC97B75730E45F44A54EADFC9").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610B59DD107321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
                tm_signature::Data {
                    r:HexDecode("4A3BF6C56CA3D066EEAEA7A254A9C7FF2CFBAB1A288444137F5CBFB125326142").unwrap(),
                    s:HexDecode("0A4C6B66AC5E5D1D46C1B9B3CABD3CE70303ACC29600D35145EAD91A425CB8D3").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("770802119CE23C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("1224080112200D565C519000654A9C0F79B4A63226C111800B9DE59777745CA2B6077A14E2162A0B0895E2F2900610F8918901321362616E642D6C616F7A692D746573746E657434").unwrap(),
                },
            ]
        };
        println!("AppendSignatureInput: {:?}", HexEncode(OBIEncode::try_to_vec(&append_signature_input).unwrap()));
    }

    #[test]
    fn test_3() {
        let verify_result_input = VerifyAndSaveResultInput {
            block_height: 3990172,
            result: result_codec::Result {
                client_id: "BSC Crypto".to_string(),
                oracle_script_id: 37,
                params: b64decode("AAAAAgAAAANWRVQAAAADWExNAAAAAAAPQkA=").unwrap(),
                ask_count: 16,
                min_count: 10,
                request_id: 1855878,
                ans_count: 14,
                request_time: 1645516981,
                resolve_time: 1645516988,
                resolve_status: 1,
                result: b64decode("AAAAAgAAAAAAAK8aAAAAAAACsag=").unwrap()
            },
            version: 3854464,
            merkle_paths: vec![
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:1,
                    sub_tree_size:2,
                    sub_tree_version:3854466,
                    sibling_hash:HexDecode("C338E83E672E59461BB966E261A8A635B058BD316EDBBDFA8294A99D211B8B8C").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:2,
                    sub_tree_size:4,
                    sub_tree_version:3854470,
                    sibling_hash:HexDecode("65A156F2F935703D4FA66667F8831344AE6F439C43DCBAD15D60445C438E04E0").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:3,
                    sub_tree_size:8,
                    sub_tree_version:3854474,
                    sibling_hash:HexDecode("C670164B4A1308D8C25B6BCF167622EF1DCF3351F16F4777610D07585310C7E0").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:4,
                    sub_tree_size:16,
                    sub_tree_version:3854491,
                    sibling_hash:HexDecode("7B11C0A227CE148E165D09FB3F0F8961ADAB57D148A43145EF03ABA8D55DC021").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:5,
                    sub_tree_size:32,
                    sub_tree_version:3854505,
                    sibling_hash:HexDecode("5B80DDDA588865B1E4255519F04FD20D78D98F7A9EF21927C9B58EFB07D1731D").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:6,
                    sub_tree_size:64,
                    sub_tree_version:3854522,
                    sibling_hash:HexDecode("7BF8FE910C5AF49D1FE3C6A34A72D14D8DF5CB6E1B283E361F03CF1083466E61").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:7,
                    sub_tree_size:128,
                    sub_tree_version:3854649,
                    sibling_hash:HexDecode("457E5DF41BF1EBF6F0279B927B94A8DF9BBE6F6B3BB30AE025692720645B71DA").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:8,
                    sub_tree_size:256,
                    sub_tree_version:3855034,
                    sibling_hash:HexDecode("CFFACF45CFB8A0AD1B0F64F7E9665A74A182D647B391A2D87FB559BBAC4B2C51").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:9,
                    sub_tree_size:511,
                    sub_tree_version:3855509,
                    sibling_hash:HexDecode("FC2041BFBF78CF37E0638CE964A4AB0A9569B4E2220BDC9003FF8D7E1F9F0EDB").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:10,
                    sub_tree_size:1023,
                    sub_tree_version:3856238,
                    sibling_hash:HexDecode("F1E73EA637D76983140A0EA001E99930A0FA5F5811DF9DCA8EB3628CBD4B449E").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:11,
                    sub_tree_size:2046,
                    sub_tree_version:3860602,
                    sibling_hash:HexDecode("2C076A69E507B3A07D90B7E251E0922223AEF85F97A878B4B186DAAC93FDBE13").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:12,
                    sub_tree_size:4094,
                    sub_tree_version:3868245,
                    sibling_hash:HexDecode("E21E786D6C0F3F6D79C7D596764E1F283FE78B4280172B2529C20072C7D533C8").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:13,
                    sub_tree_size:8165,
                    sub_tree_version:3876411,
                    sibling_hash:HexDecode("B06F652E355111218585EB1FCB83D751D0247DA83A49817FBDC17AEF41A3DD20").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:14,
                    sub_tree_size:16343,
                    sub_tree_version:3878730,
                    sibling_hash:HexDecode("7BBE548B0FC681A408F8B0B3CE902E4C02BEEA975FC430D4F29481624EA631E7").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:15,
                    sub_tree_size:32724,
                    sub_tree_version:3882299,
                    sibling_hash:HexDecode("A221D3FAD11AD0B801729DEE551FB31BD70761602D4697F4348EBBF5E0650B84").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:16,
                    sub_tree_size:65485,
                    sub_tree_version:3888753,
                    sibling_hash:HexDecode("12FDF758CE6E106B1B26B73CA062F14087710571DCEFD22EF7C98F43A91764C6").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:17,
                    sub_tree_size:128711,
                    sub_tree_version:3934330,
                    sibling_hash:HexDecode("F2AB1D6C7C408420CE176FA0F420C199CB3EDECDDDE200071EC306350F37D74C").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:19,
                    sub_tree_size:273313,
                    sub_tree_version:3990170,
                    sibling_hash:HexDecode("2359C2F3E6EE0EC5A9E9E83E64292564C2D2AD47A420EAAD4D73300027B3D0DD").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:20,
                    sub_tree_size:534600,
                    sub_tree_version:3990170,
                    sibling_hash:HexDecode("A82FAB1C31091A68E43B26FAF0CAC948F8DFB9D4D8B42076D1EF692E59BA6BA6").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:21,
                    sub_tree_size:1056169,
                    sub_tree_version:3990170,
                    sibling_hash:HexDecode("909F121A82D8AB9E3941908C51EE77DFA2A4A78108591955D2AA2C34C0D87818").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:22,
                    sub_tree_size:2075749,
                    sub_tree_version:3990171,
                    sibling_hash:HexDecode("B74695F41FE40885B30137DC1ADF037766B73B57206FA34630FF0E864B1E0ED1").unwrap()
                },
            ]
        };
        println!("VerifyResultInput: {:?}", HexEncode(OBIEncode::try_to_vec(&verify_result_input).unwrap()));
    }

    #[test]
    fn encode_test() {
        let candidate_block_input = RelayCandidateBlockInput {
            multi_store: multi_store::Data {
                auth_to_fee_grant_stores_merkle_hash: HexDecode("ACAA6420A7BB88830093A913CD39C304CACE64F2A40466CF8D08061D9B8F2485").unwrap(),
                gov_to_ibc_core_stores_merkle_hash: HexDecode("3DF354440200A45608BA6A42C7243FB0289C9E8ACF9A13F1ED27759AD0EAF404").unwrap(),
                mint_store_merkle_hash: HexDecode("06A9D989A4403F45DD2E053492260CC415C557009351850A607C8E7BAA17B0B7").unwrap(),
                oracle_iavl_state_hash: HexDecode("CB45442287E8D3662215D6ED9C1E183CB5459DB06C8855464393A005427A37D5").unwrap(),
                params_to_transfer_stores_merkle_hash: HexDecode("9A45D781D25741C42861701E1ACE5F198D4605451245C9ABC4A0E8F3D479340F").unwrap(),
                upgrade_store_merkle_hash: HexDecode("C9C8849ED125CC7681329C4D27B83B1FC8ACF7A865C9D1D1DF575CCA56F48DBE").unwrap(),
            },
            merkle_paths: block_header_merkle_path::Data {
                version_and_chain_id_hash: HexDecode("B25BE38E9445DF8411DE844C4980F1B452738BFC815BF71F49A378D3B00FF1C1").unwrap(),
                height: 826351,
                time_second: 1626771055,
                time_nano_second: 293907910,
                last_block_id_and_other: HexDecode("8FE8A8265123484F54F23797229D93550D2732E9F3BF1FC04A9B20F6B0B0BC1E").unwrap(),
                next_validator_hash_and_consensus_hash: HexDecode("670FFFC3A6123878EE2482EDE280FF8A1F17E058E089CFF0CCF8AF0BEB6709A7").unwrap(),
                last_results_hash: HexDecode("BBEFFF7E23A279218257CE0CF07EDD7A1273F714943FC97E0EDBEC3F154DE922").unwrap(),
                evidence_and_proposer_hash: HexDecode("0CBAD0DD17B60213621A85D58B58231997C19E43D5D4A2D5CBE8A33CD5D6ADC8").unwrap(),
            }
        };
        println!("RelayCandidateBlockInput: {:?}", HexEncode(OBIEncode::try_to_vec(&candidate_block_input).unwrap()));
        let append_signature_input = AppendSignatureInput {
            block_height: 826351,
            signatures: vec![
                tm_signature::Data {
                    r:HexDecode("6F2B9C8C44F161A17325529A35CE2778865B6C69058B5BD83EB11A450F1D7E91").unwrap(),
                    s:HexDecode("2A3597FEF73E65719A43D2DEA64E1722472D12C857B2260E70B6B6BDC82E6082").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("77080211EF9B0C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("12240801122064C28ED48945F64BBF5F7D641C89B0889FA7F9BE88FA5074D90197D81A1C78522A0B08F29CDA870610AD9AF60F321362616E642D6C616F7A692D746573746E657432").unwrap()
                },
                tm_signature::Data {
                    r: HexDecode("6BC382C99D0245A64DD45B7E0F8EA56CF58973DEA77A91C25B5569D17841676F").unwrap(),
                    s: HexDecode("65D1446FAE6FDE9F96C0D9241963B735858B79A859143C5163241E3F4DB1CFA0").unwrap(),
                    v: 27,
                    signed_data_prefix: HexDecode("77080211EF9B0C000000000022480A20").unwrap(),
                    signed_data_suffix: HexDecode("12240801122064C28ED48945F64BBF5F7D641C89B0889FA7F9BE88FA5074D90197D81A1C78522A0B08F29CDA870610FCE5B511321362616E642D6C616F7A692D746573746E657432").unwrap()
                },
                tm_signature::Data {
                    r:HexDecode("6D7CF8D300467A78B891342FE4F8DF96D5F923063EB3B294E0E39EC1A5064FE4").unwrap(),
                    s:HexDecode("35A348554D22EFC1EAC9C0E9301AC1D2B70B09C008F45615866CF636FD1C07FF").unwrap(),
                    v:28,
                    signed_data_prefix:HexDecode("77080211EF9B0C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("12240801122064C28ED48945F64BBF5F7D641C89B0889FA7F9BE88FA5074D90197D81A1C78522A0B08F29CDA870610A1C9F810321362616E642D6C616F7A692D746573746E657432").unwrap()
                },
                tm_signature::Data {
                    r:HexDecode("2A89628EE70EF2B6207E8210D935118489F19572D9970C775F6E172A9619F578").unwrap(),
                    s:HexDecode("23DBF8BE4A23936A4AD08AB06F77BC300E887CB21A24F2382C40B9F7BD1D54A3").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("77080211EF9B0C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("12240801122064C28ED48945F64BBF5F7D641C89B0889FA7F9BE88FA5074D90197D81A1C78522A0B08F29CDA8706109B8ACC12321362616E642D6C616F7A692D746573746E657432").unwrap()
                },
                tm_signature::Data {
                    r:HexDecode("63265D4452C227568E87388A16A05D6938479A5689F46130075C558E35E986E6").unwrap(),
                    s:HexDecode("102530E354287E0EA1A9842057B4525BF086A558F7645736E577D72C3A2BBDED").unwrap(),
                    v:27,
                    signed_data_prefix:HexDecode("77080211EF9B0C000000000022480A20").unwrap(),
                    signed_data_suffix:HexDecode("12240801122064C28ED48945F64BBF5F7D641C89B0889FA7F9BE88FA5074D90197D81A1C78522A0B08F29CDA870610E9EFE711321362616E642D6C616F7A692D746573746E657432").unwrap()
                }]
        };
        println!("AppendSignatureInput: {:?}", HexEncode(OBIEncode::try_to_vec(&append_signature_input).unwrap()));
        let verify_result_input = VerifyAndSaveResultInput {
            block_height: 826351,
            result: result_codec::Result {
                client_id: "from_scan".to_string(),
                oracle_script_id: 47,
                params: b64decode("AAAACG5ld19zZWVkAAAAAAAPQkA=").unwrap(),
                ask_count: 10,
                min_count: 10,
                request_id: 527654,
                ans_count: 10,
                request_time: 1626670501,
                resolve_time: 1626670521,
                resolve_status: 1,
                result: b64decode("AAAAQNhgFunzmurGkY73KVREj2eRoLnOLBVqakhc4f3VO5pO2iDSJR6zDiufaqgsReNGDBzPnUrNC04o+zT71Pmh0kY=").unwrap()
            },
            version: 794516,
            merkle_paths: vec![
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:1,
                    sub_tree_size:2,
                    sub_tree_version:794516,
                    sibling_hash:HexDecode("BD581C9039884C76F83C5B4CB8A0498635B95B1AF6F35B13B4CC0CDA11AD877D").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:2,
                    sub_tree_size:3,
                    sub_tree_version:794516,
                    sibling_hash:HexDecode("44A4CAB612A8E17BA549801051248D7AA59F1756B7B62FB6A8247E9FB029C9DE").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:3,
                    sub_tree_size:5,
                    sub_tree_version:794516,
                    sibling_hash:HexDecode("629444F42963B8AB46FB6579F5F904C4C964B7D61A5608D9E91680AD020AECC4").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:4,
                    sub_tree_size:9,
                    sub_tree_version:794516,
                    sibling_hash:HexDecode("0DAFB2AE6455293750B8FBD5D10AD7FF5630CD508B064A171EB5949A855CDB5F").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:5,
                    sub_tree_size:25,
                    sub_tree_version:794523,
                    sibling_hash:HexDecode("97857481B07D60CA72A80A1DE9D97D4848450B4B5341B6788C6D77C5A87DA8C5").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:6,
                    sub_tree_size:56,
                    sub_tree_version:794533,
                    sibling_hash:HexDecode("E6244ECB708D37EA1C916E1EF668FEFAB213C85B96816626830BFBDE9C71CD86").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:7,
                    sub_tree_size:114,
                    sub_tree_version:794555,
                    sibling_hash:HexDecode("B04DB6B3FFDA68CB81AFB9615E6DDC4BE3751B5F802D84B874432AD7E8475CFC").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:8,
                    sub_tree_size:223,
                    sub_tree_version:794590,
                    sibling_hash:HexDecode("302B227F6FFD0A99BE5E0774B7FFBB74D4171C8B9C82434D3910CFF0FEC16D4F").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:9,
                    sub_tree_size:455,
                    sub_tree_version:794675,
                    sibling_hash:HexDecode("6C80FE9448CEC35B4E1444347674362BF511F25A4BD9954A9D425AFF4999D739").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:10,
                    sub_tree_size:921,
                    sub_tree_version:795173,
                    sibling_hash:HexDecode("53FC8CE0AEB2CF7126408F0C34F999E01BF3EA456488BF99A69F3480C6B276C6").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:11,
                    sub_tree_size:1859,
                    sub_tree_version:796182,
                    sibling_hash:HexDecode("A3B4A3726C9F69D3615CCC2A358DC662C7FBE31A4E4AAFFCA5EF5D98B29244A7").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:12,
                    sub_tree_size:3720,
                    sub_tree_version:796882,
                    sibling_hash:HexDecode("7A94916148BACF4E19AE36E6055D4E1A9E6C4A4F7601AD29E83A57D8DD74D419").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:13,
                    sub_tree_size:7527,
                    sub_tree_version:800947,
                    sibling_hash:HexDecode("4A6EA9C54A229E4FFB6535B02B67745B07F00159C0EA23E5334545A2E4A058C0").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:14,
                    sub_tree_size:15053,
                    sub_tree_version:808939,
                    sibling_hash:HexDecode("A7E219B9C4684A0F61B9306485AC90700C707912467BE1815149276148A72F21").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:15,
                    sub_tree_size:30233,
                    sub_tree_version:814286,
                    sibling_hash:HexDecode("8BE3C670A74E7ACFF15C25684456CD38EF672607F04A6CA2482631D584E2ACDF").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:16,
                    sub_tree_size:60396,
                    sub_tree_version:825063,
                    sibling_hash:HexDecode("8AC27ED9C31DC9291BD90A4278E0E52CB3711D91336B2A1C82292B76E1FAB914").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:false,
                    sub_tree_height:17,
                    sub_tree_size:91995,
                    sub_tree_version:826350,
                    sibling_hash:HexDecode("1008ECCF8008F6B3F648A05E6A546D4CA1294A1DD2817502A229A411EDA3F880").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:18,
                    sub_tree_size:212397,
                    sub_tree_version:826350,
                    sibling_hash:HexDecode("939EC419B4857E138A26F8E3003E3190F94B63E0273A4ED119D258D39AFD5FCC").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:19,
                    sub_tree_size:332706,
                    sub_tree_version:826350,
                    sibling_hash:HexDecode("ECFCCA113EFCDCB23C504EF173643EA0DB0576D4E41AD17B5903D6CBD2F11767").unwrap()
                },
                iavl_merkle_path::Data {
                    is_data_on_right:true,
                    sub_tree_height:20,
                    sub_tree_size:573112,
                    sub_tree_version:826350,
                    sibling_hash:HexDecode("8078CA2F9045BD928571AC33EEFD5FD1386129A8F450C657049534DDAD7E476C").unwrap()
                }
            ]
        };
        println!("VerifyResultInput: {:?}", HexEncode(OBIEncode::try_to_vec(&verify_result_input).unwrap()));
    }
}
