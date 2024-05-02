use crate::{ItemSynNodePath, PreludeError};
use husky_entity_path::path::{major_item::ty::TypePath, PrincipalEntityPath};
use husky_token::{IdentToken, PathNameToken, SuperToken};
use husky_vfs::{error::VfsError, ModulePath, ToolchainError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub enum EntityTreeError {
    #[error("original {0}")]
    Original(#[from] OriginalEntityTreeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedEntityTreeError),
}

impl From<PreludeError> for EntityTreeError {
    fn from(e: PreludeError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq, Clone)]
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
    #[error("NoSubitemForForm")]
    NoSubitemForForm,
    #[error("InvalidParentPath")]
    InvalidParentPath {
        name_token: PathNameToken,
        principal_entity_path: PrincipalEntityPath,
    },
    #[error("StandaloneSelf")]
    StandaloneSelf,
    #[error("SameNameTypeItemsButNotTheSameKind")]
    SameNameTypeItemsButNotTheSameKind,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
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
