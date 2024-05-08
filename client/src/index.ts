import * as protobufjs from 'protobufjs';
import * as message from '../../message.proto';
protobufjs.load("../message.proto").then(()=>{
    console.log("done");
})
console.log("hello world");