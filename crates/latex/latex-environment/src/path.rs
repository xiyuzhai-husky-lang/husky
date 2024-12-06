pub mod menu;

use base_coword::BaseCoword;
use eterned::db::EternerDb;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LxEnvironmentPath {
    name: LxEnvironmentName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LxEnvironmentName(BaseCoword);

impl LxEnvironmentPath {
    pub fn new(name: &str, db: &EternerDb) -> Self {
        Self {
            name: LxEnvironmentName::from_ref(name, db),
        }
    }
}

impl LxEnvironmentPath {
    pub fn name(&self) -> LxEnvironmentName {
        self.name
    }
}

impl LxEnvironmentName {
    pub fn new(coword: BaseCoword) -> Self {
        Self(coword)
    }

    pub fn from_ref(name: &str, db: &EternerDb) -> Self {
        Self(BaseCoword::from_ref(name, db))
    }
}

impl LxEnvironmentName {
    pub fn coword(self) -> BaseCoword {
        self.0
    }
}
