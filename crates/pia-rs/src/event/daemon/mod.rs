mod data;
mod state;
pub use data::*;
pub use state::*;

use serde_derive::{Deserialize, Serialize};
mod util;
pub use util::{OptionalIpv4Addr, UnixTime};

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
pub enum DaemonEvent {
    Data([data::DataEventParam; 1]),
}
