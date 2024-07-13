use std::{collections::HashMap, sync::{RwLock, RwLockReadGuard}};

use crate::{ConstString, ServerCode};


static LANGUAGE_REGISTRY: RwLock<LanguageRegistry> = RwLock::new(LanguageRegistry::null());

pub fn language_registry() -> RwLockReadGuard<'static, LanguageRegistry> {
    LANGUAGE_REGISTRY.read().expect("rwlock poisoned")
}

pub struct LanguageRegistry {
    pub code: LanguageCode,
    pub server_displays: HashMap<ServerCode, ServerDisplay>,
}
impl LanguageRegistry {
    const fn null() -> Self {
        todo!()
    }
}

pub struct ServerDisplay {
    pub name: ConstString,
    pub prefix: Option<ConstString>
}


#[derive(Debug, Clone, Hash)]
pub struct LanguageCode(ConstString);

