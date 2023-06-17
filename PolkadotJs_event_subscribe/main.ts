/*实现事件订阅通知*/
import { ApiPromise, WsProvider } from "@polkadot/api";

const WEB_SOCKET = "ws://localhost:64361";

const connectSubstrateApi = async () => {
    const provider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({provider});
    await api.isReady;
    console.log("api is ready");
    return api;
};

const eventSubscribe = async (api: ApiPromise) => {
    await api.query.system.events((events: any[]) => {
        // Loop through the Vec<EventRecord>
        events.forEach((record: { event: any; phase: any; }) => {
            const {event, phase} = record;
            const types = event.typeDef;
            // Loop through each of the parameters, displaying the type and data
            event.data.forEach((data: { toString: () => any; }, index: string | number) => {
                console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
            });
        }
    )});
};


const main = async () => {
    const connectApi = await connectSubstrateApi();
    await eventSubscribe(connectApi);
}

main().then(() => {
    console.log("main function is done");
}
).catch((err) => {
    console.log("main function is error, error is ", err);
}
);


