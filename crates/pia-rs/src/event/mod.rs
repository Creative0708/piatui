pub mod data;
use serde_derive::{Deserialize, Serialize};
mod util;
pub use util::UnixTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct PIADaemonEvent {
    #[serde(rename = "jsonrpc")]
    jsonrpc_version: String,

    #[serde(flatten)]
    event: PIADaemonEventInner,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "lowercase")]
pub enum PIADaemonEventInner {
    Data([data::DataEventParam; 1]),
}
