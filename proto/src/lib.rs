mod proto {
    include!(concat!(env!("OUT_DIR"), "/messages.rs"));
}

pub use proto::*;
pub use prost;