use husky_entity_path::EntityPathError;
use husky_vfs::VfsError;
use thiserror::Error;

use crate::EntitySymbolDb;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntitySymbolError {
    // original
    #[error("todo")]
    TODO,
    #[error("expect identifier after keyword")]
    ExpectIdentifierAfterKeyword,
    // derived
    #[error("derived {0}")]
    DerivedSelf(Box<Self>),
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    EntityPath(#[from] EntityPathError),
}

impl From<&EntitySymbolError> for EntitySymbolError {
    fn from(value: &EntitySymbolError) -> Self {
        EntitySymbolError::DerivedSelf(Box::new(value.clone()))
    }
}

pub type EntitySymbolResult<T> = Result<T, EntitySymbolError>;

impl salsa::DebugWithDb<dyn EntitySymbolDb + '_> for EntitySymbolError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntitySymbolDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: EntitySymbolDb> salsa::DebugWithDb<Db> for EntitySymbolError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntitySymbolDb, include_all_fields)
    }
}
