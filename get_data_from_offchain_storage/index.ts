import {ApiPromise, WsProvider} from '@polkadot/api';

const {u8aToString} = require("@polkadot/util");

const SOCKET_URL = 'ws://127.0.0.1:9944';

const connectSubstrate = async () => {
    const wsProvider = new WsProvider(SOCKET_URL);
    const api = await ApiPromise.create({provider: wsProvider});
    await api.isReady;
    console.log('Connection to Substrate is OK.')
    return api;
}

const ONCHAIN_TX_KEY = "my_pallet::indexing1";

const readDataFromOffChainStorage = async (api: ApiPromise) => {
    const result = await api.rpc.offchain.localStorageGet(
        'PERSISTENT',
        ONCHAIN_TX_KEY
    );
    const hexValue = result.toHex();
    const u8aValue = new Uint8Array(
        (hexValue.match(/.{1,2}/g) || []).map((byte) => parseInt(byte, 16))
    );
    
    const stringValue = u8aToString(u8aValue);
    console.log("value in offChain Storage is", stringValue);
}
/// 主函数
const main = async () => {
    const api = await connectSubstrate();
    await readDataFromOffChainStorage(api);
}
main()
    .then(() => {
        console.log('successfully exited');
        process.exit(0);
    })
    .catch(err => {
        console.log('error occur:', err);
        process.exit(1);
    })