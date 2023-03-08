use husky_ast::AstIdx;
use husky_entity_path::EntityPathError;
use husky_manifest::ManifestError;
use husky_vfs::{ModulePath, ToolchainError, VfsError};
use thiserror::Error;

use crate::{EntityTreeBundleError, EntityTreeDb, PreludeError};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum EntityTreeError {
    // original
    #[error("todo")]
    TODO,
    #[error("expect identifier after keyword")]
    ExpectIdentAfterKeyword,
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
    #[error("no subentity")]
    NoSubentity,
    #[error("from bundle {0}")]
    CrateBundle(#[from] EntityTreeBundleError),
    #[error("unresolved identifier")]
    UnresolvedIdent(husky_token::IdentToken),
    #[error("SymbolNotAccessible")]
    SymbolNotAccessible,
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
