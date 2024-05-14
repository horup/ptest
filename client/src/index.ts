import * as proto from '../../proto/message';
let ws:WebSocket|null = null;
function connect() {
    if (ws != null) {
        ws.onclose = null;
        ws.onopen = null;
        ws.onerror = null;
        ws = null;
    }
    ws = new WebSocket("ws://localhost:8080");
    ws.onopen = ()=>onConnected();
    ws.onmessage = async (msg)=>await onMessage(msg);
    ws.onclose = ()=>{
        setTimeout(() => {
            connect();
        }, 1000);
        onDisconnected();
    }
}
connect();

async function onMessage(e:MessageEvent<any>) {
    let blob = e.data as Blob;
    let data = (await blob.stream().getReader().read()).value;
    if (data) {
        let msg = proto.Message.decode(data); 
        console.log(msg);
    }
}
function onConnected() {
    console.log("connected");
    let msg = proto.Message.create({
        join:{
            id:crypto.randomUUID(),
            name:"Test Player"
        }
    });
    sendMsg(msg);
}

function sendMsg(msg:proto.Message) {
    let encoded = proto.Message.encode(msg).finish();
    ws?.send(encoded);
}

function onDisconnected() {
    console.log("disconnected");
}

function update() {
    requestAnimationFrame(()=>update());
}

async function init() {
    update();
}

init();

(window as any).cmd = {
    "createInstance":()=>{
        let msg = proto.Message.create({
            createInstance:{
                name:"Test Name"
            }
        });
        sendMsg(msg);
    },
    "refreshLobby":()=>{
        sendMsg({
            refreshLobby:{}
        });
    }
}