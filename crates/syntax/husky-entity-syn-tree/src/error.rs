use crate::{EntitySynNode, EntitySynTreeDb, EntityTreeBundleError, PreludeError};
use husky_ast::AstIdx;
use husky_entity_path::{EntityPathError, TypePath};
use husky_manifest::ManifestError;
use husky_token::{IdentToken, TokenIdx};
use husky_vfs::{ModulePath, ToolchainError, VfsError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum EntityTreeError {
    #[error("original {0}")]
    Original(#[from] OriginalEntityTreeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedEntityTreeError),
}

impl From<&PreludeError> for EntityTreeError {
    fn from(e: &PreludeError) -> Self {
        todo!()
    }
}

impl From<PreludeError> for EntityTreeError {
    fn from(e: PreludeError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

impl From<EntityTreeBundleError> for EntityTreeError {
    fn from(e: EntityTreeBundleError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

impl From<&EntityTreeBundleError> for EntityTreeError {
    fn from(e: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<VfsError> for EntityTreeError {
    fn from(e: VfsError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

impl From<&EntityTreeError> for EntityTreeError {
    fn from(e: &EntityTreeError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum OriginalEntityTreeError {
    #[error("unresolved root identifier")]
    UnresolvedRootIdent(IdentToken),
    #[error("no visible subitem")]
    NoVisibleSubitem,
    #[error("item symbol already defined")]
    EntitySymbolAlreadyDefined {
        old: EntitySynNode,
        new: EntitySynNode,
    },
    #[error("expect identifier after keyword")]
    ExpectIdentAfterKeyword,
    #[error("InvalidTypePath")]
    InvalidTypePath(TypePath),
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
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
    #[error("from bundle {0}")]
    CrateBundle(#[from] EntityTreeBundleError),
    #[error("invalid module path")]
    InvalidModulePath(ModulePath),
}

pub type ItemSynTreeResult<T> = Result<T, EntityTreeError>;
pub type EntityTreeResultRef<'a, T> = Result<T, &'a EntityTreeError>;
