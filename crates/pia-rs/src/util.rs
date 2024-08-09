use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

use serde_derive::{Deserialize, Serialize};

/// Like `String`, but smaller and cheaper to clone.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ConstString(Arc<str>);

impl Deref for ConstString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Debug for ConstString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&**self, f)
    }
}
impl fmt::Display for ConstString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&**self, f)
    }
}
impl<'de> serde::Deserialize<'de> for ConstString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = <&str as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self::from(str))
    }
}
impl serde::Serialize for ConstString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <str as serde::Serialize>::serialize(&**self, serializer)
    }
}
impl From<&str> for ConstString {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct ServerCode(ConstString);

pub type ServerMap<T> = HashMap<ServerCode, T>;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct CountryCode(ConstString);

pub type CountryMap<T> = HashMap<CountryCode, T>;
