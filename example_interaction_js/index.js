const {
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
  StdFee,
} = require("@terra-money/terra.js");
const {
  encodeRelayCandidateBlockInput,
  encodeAppendSignatureInput,
  encodeVerifyAndSaveResultInput,
  encodeCalldata,
} = require("./utils.js");
const axios = require("axios");
const { Client, Transaction, Message, Wallet } = require("@bandprotocol/bandchain.js");
const { MsgRequestData } = Message;
const { PrivateKey } = Wallet;

// terra constants
const bridgeAddress = "terra1l9drxzsmxrlspm73wurxnptsawyrn3s63k7qd4";
const consumerAddress = "terra1y6t238vszfgsgsztxmzger3lya8e22fknfjcrd";
const terraTestMnemonic = "xxx";

// band constants
const bandchain = new Client("http://rpc-laozi-testnet2.bandchain.org:8080");
const bandTestMnemonic = "xxx"

// gas limit
const GAS = 2_000_000;

// connect to tequila testnet
const terra = new LCDClient({
  URL: "https://tequila-lcd.terra.dev",
  chainID: "tequila-0004",
});

const band_requester_privkey = PrivateKey.fromMnemonic(bandTestMnemonic);
const band_requester_pubkey = band_requester_privkey.toPubkey();
const band_requester_address = band_requester_pubkey.toAddress();

const wallet = terra.wallet(new MnemonicKey({ mnemonic: terraTestMnemonic }));

const sleep = async (ms) => new Promise((r) => setTimeout(r, ms));

console.log("xcxcxcxcxcxcxc")
process.exit();

const relayAndVerify = async (proof) => {
  const encodedBlockHeader = encodeRelayCandidateBlockInput(proof);
  const encodedSigs = encodeAppendSignatureInput(proof);
  const encodeVerifyAndSaveResult = encodeVerifyAndSaveResultInput(proof);

  // create msgs
  const msg1 = new MsgExecuteContract(wallet.key.accAddress, bridgeAddress, {
    relay_candidate_block: { data: encodedBlockHeader },
  });
  const msg2 = new MsgExecuteContract(wallet.key.accAddress, bridgeAddress, {
    append_signature: { data: encodedSigs },
  });
  const msg3 = new MsgExecuteContract(wallet.key.accAddress, bridgeAddress, {
    verify_and_save_result: { data: encodeVerifyAndSaveResult },
  });
  const msg4 = new MsgExecuteContract(wallet.key.accAddress, consumerAddress, {
    save_verified_result: { request_id: parseInt(proof.oracle_data_proof.result.request_id, 10) }
  });

  // sign tx
  const signedTx = await wallet.createAndSignTx({
    msgs: [msg1, msg2, msg3, msg4],
    fee: new StdFee(GAS, { uluna: Math.ceil(GAS * 0.3) }),
  });

  // broadcast tx
  const { txhash } = await terra.tx.broadcastSync(signedTx);
  console.log("broadcast tx to terra chain: ", txhash);

  const txResult = await validateTx(txhash);
  console.log("\n");
  if (!txResult) {
    throw "Fail to get result from chain";
  }
};

const requestDataAndGetProof = async () => {
  try {
    const band_account = await bandchain.getAccount(band_requester_address.toAccBech32());
    const chain_id = await bandchain.getChainId();

    // https://laozi-testnet2.cosmoscan.io/oracle-script/50
    const oracle_script_id = 50;

    // Example calldata
    const path = "league/223.l.431/players;player_keys=223.p.5479";
    const keys = "league,1,players,0,player,0,2";
    const calldata = encodeCalldata(path, keys);

    console.log('Submitting request to BandChain');
    const tx = new Transaction()
        .withMessages(
            new MsgRequestData(
              oracle_script_id,
              Buffer.from(calldata, "hex"),
              4,
              3,
              "from_yahoo_fantasy_example",
              band_requester_address.toAccBech32(),
            ).toAny()
        )
        .withAccountNum(band_account.accountNumber)
        .withSequence(band_account.sequence)
        .withChainId(chain_id)
        .withGas(1000000)
        .withMemo("Yahoo fantasy sportsdata bridge example");

    const signDoc = tx.getSignDoc(band_requester_pubkey);
    const signature = band_requester_privkey.sign(signDoc);
    const txRawBytes = tx.getTxData(signature, band_requester_pubkey)

    const txResult = await bandchain.sendTxBlockMode(txRawBytes);
    console.log("txHash:", txResult.txhash);

    const [requestID] = await bandchain.getRequestIdByTxHash(txResult.txhash);
    console.log("Request ID:", requestID);

    let proof;
    let max_retry = 10;
    while (max_retry > 0) {
      max_retry--;
      try {
        const result = await axios.get(
          "https://laozi-testnet2.bandchain.org/oracle/proof/" + requestID
        );
        if (result.status !== 200) {
          await sleep(2000);
        } else {
          proof = result.data.result.proof;
          break;
        }
      } catch(err) {
        if (err.isAxiosError && err.response && err.response.status !== 404) {
          console.error(err.response.data);
        } else if (!err.isAxiosError) {
          console.error(err.message);
        }
        await sleep(2000);
      }
    }
    return proof;
  } catch (e) {
    console.log(e);
    return null;
  }
};

const validateTx = async (txhash) => {
  let max_retry = 30;
  while (max_retry > 0) {
    await sleep(1000);
    max_retry--;
    try {
      process.stdout.clearLine();
      process.stdout.cursorTo(0);
      process.stdout.write("polling: " + (30 - max_retry));
      const txInfo = await terra.tx.txInfo(txhash);
      return txInfo;
    } catch (err) {
      if (err.isAxiosError && err.response && err.response.status !== 404) {
        console.error(err.response.data);
      } else if (!err.isAxiosError) {
        console.error(err.message);
      }
    }
  }
  return null;
};

const getLatestSaveResultFromConsumer = async () => {
  try {
    const result = await terra.wasm.contractQuery(consumerAddress, {
      latest_saved_result: {},
    });
    return result;
  } catch (e) {
    console.log("Fail to get latest result from consumer contract");
    console.log(e);
  }
  return null;
};

// Main Flow
(async () => {
  try {
    console.log(
      "=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    );

    // 1. request data on Bandchain and then get its proof of existence
    const result = await requestDataAndGetProof();
    console.log(result);

    // 2. relay the data with its proof to Band's bridge contrcat on Terra
    // and then let the consumer contract to consume the data from Band's bridge contract
    await relayAndVerify(result);

    // 3. try to read the latest saved result from the consumer contract
    const currentRates = await getLatestSaveResultFromConsumer();
    if (currentRates) {
      console.log("latest saved result: ", JSON.stringify(currentRates));
    } else {
      throw "Fail to get current rates from std contract";
    }
  } catch (e) {
    console.log(e);
  }
  console.log(
    "=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
  );
})();