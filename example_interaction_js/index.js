const {
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
} = require("@terra-money/terra.js");
const {
  encodeRelayCandidateBlockInput,
  encodeAppendSignatureInput,
  encodeVerifyAndSaveResultInput,
  encodeCalldata,
} = require("./utils.js");
const axios = require("axios");
const { Client, Transaction, Message, Wallet, Obi, Coin, Fee } = require("@bandprotocol/bandchain.js");
const { MsgRequestData } = Message;
const { PrivateKey } = Wallet;

// terra constants
const bridgeAddress = "terra1l9drxzsmxrlspm73wurxnptsawyrn3s63k7qd4";
const consumerAddress = "terra1y6t238vszfgsgsztxmzger3lya8e22fknfjcrd";
const terraTestMnemonic = process.env.TERRA_MNEMONIC;

// band constants
const bandchain = new Client("https://laozi-testnet4.bandchain.org/grpc-web");
const bandTestMnemonic = process.env.BAND_MNEMONIC;

// gas limit
const GAS = 2_000_000;

// connect to tequila testnet
const terra = new LCDClient({
  URL: "https://bombay-lcd.terra.dev",
  chainID: "bombay-12",
  gasPrices: { uluna: 0.38 }
});

const band_requester_privkey = PrivateKey.fromMnemonic(bandTestMnemonic);
const band_requester_pubkey = band_requester_privkey.toPubkey();
const band_requester_address = band_requester_pubkey.toAddress();

const wallet = terra.wallet(new MnemonicKey({ mnemonic: terraTestMnemonic }));

const sleep = async (ms) => new Promise((r) => setTimeout(r, ms));

const relayAndVerify = async (proof) => {
  const encodedBlockHeader = encodeRelayCandidateBlockInput(proof);
  const encodedSigs = encodeAppendSignatureInput(proof);
  const encodeVerifyAndSaveResult = encodeVerifyAndSaveResultInput(proof);

  // create msgs
  const msg1 = new MsgExecuteContract(wallet.key.accAddress, bridgeAddress, {
    relay_candidate_block: { data: encodedBlockHeader },
  });
  const msg2 = encodedSigs.map( sigs => new MsgExecuteContract(wallet.key.accAddress, bridgeAddress, {
    append_signature: { data: sigs },
  }));
  const msg3 = new MsgExecuteContract(wallet.key.accAddress, bridgeAddress, {
    verify_and_save_result: { data: encodeVerifyAndSaveResult },
  });
  const msg4 = new MsgExecuteContract(wallet.key.accAddress, consumerAddress, {
    save_verified_result: { request_id: parseInt(proof.oracle_data_proof.result.request_id, 10) }
  });

  // sign tx
  const signedTx = await wallet.createAndSignTx({ msgs: [msg1, ...msg2, msg3, msg4], memo: "from example interaction" });

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
    const sender = band_requester_address.toAccBech32();

    // https://laozi-testnet4.cosmoscan.io/oracle-script/134
    const oracle_script_id = 134;

    // Example calldata
    const obi = new Obi('{sliced_index_input:i8}/{result:string}');
    const calldata = obi.encodeInput({ sliced_index_input: 0 });

    let coin = new Coin()
    coin.setDenom('uband')
    coin.setAmount('1000000')

    let feeCoin = new Coin();
    feeCoin.setDenom('uband');
    feeCoin.setAmount('1000');

    console.log('Submitting request to BandChain');

    const requestMessage = new MsgRequestData(
          oracle_script_id,
          calldata,
          4,
          3,
          "from_example_interaction_script",
          sender,
          [coin],
          50000,
          200000,
    );

    const fee = new Fee()
    fee.setAmountList([feeCoin])
    fee.setGasLimit(1000000)
    const chainId = await bandchain.getChainId()
    const txn = new Transaction()
    txn.withMessages(requestMessage)
    await txn.withSender(bandchain, sender)
    txn.withChainId(chainId)
    txn.withFee(fee)
    txn.withMemo('')

    const signDoc = txn.getSignDoc(band_requester_pubkey)
    const signature = band_requester_privkey.sign(signDoc)
    const txRawBytes = txn.getTxData(signature, band_requester_pubkey)

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
          "https://laozi-testnet4.bandchain.org/api/oracle/proof/" + requestID
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