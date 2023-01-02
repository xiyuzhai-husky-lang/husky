use husky_ast::AstIdx;
use husky_entity_path::EntityPathError;
use husky_manifest::ManifestError;
use husky_vfs::{ModulePath, ToolchainError, VfsError};
use thiserror::Error;

use crate::{EntityTreeBundleError, EntityTreeDb, PreludeError};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeError {
    // original
    #[error("todo")]
    TODO,
    #[error("expect identifier after keyword")]
    ExpectIdentifierAfterKeyword,
    // derived
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    EntityPath(#[from] EntityPathError),
    #[error("derived {0}")]
    Manifest(#[from] ManifestError),
    #[error("entity symbol already defined, old = {old}, new = {new}")]
    EntitySymbolAlreadyDefined { old: AstIdx, new: AstIdx },
    #[error("invalid module path")]
    InvalidModulePath(ModulePath),
    #[error("from toolchain error {0}")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error {0}")]
    Prelude(#[from] PreludeError),
    #[error("from bundle {0}")]
    Bundle(#[from] EntityTreeBundleError),
}

// impl From<&EntityTreeError> for EntityTreeError {
//     fn from(value: &EntityTreeError) -> Self {
//         EntityTreeError::DerivedSelf(Box::new(value.clone()))
//     }
// }

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntityTreeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityTreeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}
