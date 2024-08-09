pub mod data;
pub mod state;
use serde_derive::{Deserialize, Serialize};
mod util;
pub use util::UnixTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct DaemonEvent {
    #[serde(rename = "jsonrpc")]
    pub jsonrpc_version: String,

    #[serde(flatten)]
    pub event: Box<DaemonEventInner>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "lowercase")]
pub enum DaemonEventInner {
    Data([data::DataEventParam; 1]),
}
