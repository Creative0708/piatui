
use serde_derive::{Deserialize, Serialize};
mod data;
mod util;
pub use util::{ServerCode, ConstString};
mod lang;

#[derive(Deserialize, Serialize)]
pub struct PIADaemonEvent {
    #[serde(rename = "jsonrpc")]
    jsonrpc_version: String,

    #[serde(flatten)]
    event: PIADaemonEventInner,
}

#[derive(Deserialize, Serialize)]
pub enum PIADaemonEventInner {
    DataEvent(data::DataEventParam),
}