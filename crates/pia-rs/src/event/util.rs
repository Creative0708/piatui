use std::{
    fmt::Debug,
    net,
    str::FromStr,
    time::{Duration, SystemTime},
};

use serde::{de, ser};
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

/// PIA uses "" instead of null for nonexistent IP addresses. :(
#[derive(Debug, Clone, Copy)]
pub struct OptionalIpv4Addr(pub Option<net::Ipv4Addr>);

impl ser::Serialize for OptionalIpv4Addr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(addr) => addr.serialize(serializer),
            None => "".serialize(serializer),
        }
    }
}
impl<'de> de::Deserialize<'de> for OptionalIpv4Addr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Ok(if s.is_empty() {
            Self(None)
        } else {
            Self(Some(net::Ipv4Addr::from_str(s).map_err(de::Error::custom)?))
        })
    }
}
