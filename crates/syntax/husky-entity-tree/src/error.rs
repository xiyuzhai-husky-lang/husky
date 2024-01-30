use crate::{ItemSynNodePath, PreludeError};
use husky_entity_path::{EntityPathError, TypePath};
use husky_token::{IdentToken, SuperToken};
use husky_vfs::{error::VfsError, ModulePath, ToolchainError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db]
pub enum EntityTreeError {
    #[error("original {0}")]
    Original(#[from] OriginalEntityTreeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedEntityTreeError),
}

impl From<&PreludeError> for EntityTreeError {
    fn from(_e: &PreludeError) -> Self {
        todo!()
    }
}

impl From<PreludeError> for EntityTreeError {
    fn from(e: PreludeError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

impl From<VfsError> for EntityTreeError {
    fn from(e: VfsError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db]
pub enum OriginalEntityTreeError {
    #[error("unresolved root identifier")]
    UnresolvedRootIdent(IdentToken),
    #[error("no visible subitem")]
    NoVisibleSubitem,
    #[error("item symbol already defined")]
    EntitySymbolAlreadyDefined {
        old: ItemSynNodePath,
        new: ItemSynNodePath,
    },
    #[error("expect identifier after keyword")]
    ExpectIdentAfterKeyword,
    #[error("InvalidTypePath")]
    InvalidTypePath(TypePath),
    #[error("CanOnlyUseParentSuperNotForModulePath")]
    CanOnlyUseParentSuperForModulePath,
    #[error("NoSuperForCrateRoot")]
    NoSuperForCrateRoot { super_token: SuperToken },
    #[error("NoSubitemForFugitive")]
    NoSubitemForFugitive,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db]
pub enum DerivedEntityTreeError {
    // derived
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    EntityPath(#[from] EntityPathError),
    #[error("derived")]
    Manifest,
    #[error("from toolchain error {0}")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error {0}")]
    Prelude(#[from] PreludeError),
    #[error("invalid module path")]
    InvalidModulePath(ModulePath),
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
pub type EntityTreeResultRef<'a, T> = Result<T, &'a EntityTreeError>;
