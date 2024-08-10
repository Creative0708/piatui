use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
pub enum ClientEvent {
    ConnectVPN,
    DisconnectVPN,
}
