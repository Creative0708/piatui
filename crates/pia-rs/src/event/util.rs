use std::{
    fmt::Debug,
    time::{Duration, SystemTime},
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct UnixTime(pub u64);

impl Debug for UnixTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnixTime")
            .field(
                match SystemTime::UNIX_EPOCH.checked_add(Duration::from_millis(self.0)) {
                    Some(ref time) => time,
                    None => &self.0,
                },
            )
            .finish()
    }
}
