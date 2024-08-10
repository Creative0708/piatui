use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct ServerCode(String);

pub type ServerMap<T> = HashMap<ServerCode, T>;

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub struct CountryCode(String);

pub type CountryMap<T> = HashMap<CountryCode, T>;
