use crate::{EntityTreeBundleError, EntityTreeDb, NativeEntitySymbol, PreludeError};
use husky_ast::AstIdx;
use husky_entity_path::EntityPathError;
use husky_manifest::ManifestError;
use husky_token::{IdentToken, TokenIdx};
use husky_vfs::{ModulePath, ToolchainError, VfsError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
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

impl From<EntityTreeBundleError> for EntityTreeError {
    fn from(e: EntityTreeBundleError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

impl From<VfsError> for EntityTreeError {
    fn from(e: VfsError) -> Self {
        EntityTreeError::Derived(e.into())
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum OriginalEntityTreeError {
    #[error("unresolved identifier")]
    UnresolvedIdent(IdentToken),
    #[error("SymbolNotAccessible")]
    SymbolNotAccessible(IdentToken),
    #[error("no subentity")]
    NoSubentity,
    #[error("entity symbol already defined")]
    EntitySymbolAlreadyDefined {
        old: NativeEntitySymbol,
        new: NativeEntitySymbol,
    },
    #[error("expect identifier after keyword")]
    ExpectIdentAfterKeyword,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum DerivedEntityTreeError {
    // derived
    #[error("derived {0}")]
    Vfs(#[from] VfsError),
    #[error("derived {0}")]
    EntityPath(#[from] EntityPathError),
    #[error("derived {0}")]
    Manifest(#[from] ManifestError),
    #[error("from toolchain error {0}")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error {0}")]
    Prelude(#[from] PreludeError),
    #[error("from bundle {0}")]
    CrateBundle(#[from] EntityTreeBundleError),
    #[error("invalid module path")]
    InvalidModulePath(ModulePath),
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
