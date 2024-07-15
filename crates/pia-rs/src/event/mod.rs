mod data;
use serde_derive::{Deserialize, Serialize};

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
