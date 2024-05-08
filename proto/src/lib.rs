mod proto {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

impl netcode::SerializableMessage for proto::Message {
    fn to_bytes(&self) -> Result<Vec<u8>, ()> {
        Ok(prost::Message::encode_to_vec(self))
    }

    fn from_bytes(bytes:&[u8]) -> Result<Self, ()> {
        prost::Message::decode(bytes).map_err(|_|())
    }

    fn to_json(&self) -> Result<String, ()> {
        panic!("not supported!");
    }

    fn from_json(_json:&str) -> Result<Self, ()> {
        panic!("not supported!")
    }
}

pub use proto::*;