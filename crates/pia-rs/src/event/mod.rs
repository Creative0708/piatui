use serde_derive::{Deserialize, Serialize};

pub mod client;
pub mod daemon;

#[derive(Deserialize, Serialize, Debug)]
pub struct JSONRPCMessage<Inner> {
    #[serde(rename = "jsonrpc")]
    pub jsonrpc_version: String,

    #[serde(flatten)]
    pub event: Box<Inner>,
}
