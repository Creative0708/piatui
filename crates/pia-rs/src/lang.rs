use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard},
};

use serde_derive::{Deserialize, Serialize};

use crate::{ConstString, ServerCode};

// No support for languages other than en-US for now, unfortunately :(
// this is just a scaffold so far
static LANGUAGE_REGISTRY: RwLock<Option<LanguageRegistry>> = RwLock::new(None);

// pub fn language_registry() -> RwLockReadGuard<'static, LanguageRegistry> {
//     LANGUAGE_REGISTRY.read().expect("rwlock poisoned")
// }

pub struct LanguageRegistry {
    pub code: LanguageCode,
    pub server_displays: HashMap<ServerCode, ServerDisplay>,
}
impl LanguageRegistry {}

pub struct ServerDisplay {
    pub name: ConstString,
    pub prefix: Option<ConstString>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct LanguageCode(pub ConstString);
