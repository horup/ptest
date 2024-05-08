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
    ws.onmessage = (msg)=>onMessage(msg);
    ws.onclose = ()=>{
        setTimeout(() => {
            connect();
        }, 1000);
        onDisconnected();
    }
}
connect();

function onMessage(msg:MessageEvent<any>) {
    console.log(msg);
}
function onConnected() {
    console.log("connected");
    let msg = proto.Message.create({
        join:{
            playerId:crypto.randomUUID()
        }
    });
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