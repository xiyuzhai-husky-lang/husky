use crate::{
    EntitySynTreeDb, EntitySynTreeJar, EntityTreeBundleError, ItemSynNodePath, PreludeError,
};
use husky_entity_path::{EntityPathError, TypePath};
use husky_token::IdentToken;
use husky_vfs::{error::VfsError, ModulePath, ToolchainError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
#[deprecated(note = "put this into syntax error")]
pub enum EntitySynTreeError {
    #[error("original {0}")]
    Original(#[from] OriginalEntityTreeError),
    #[error("derived {0}")]
    Derived(#[from] DerivedEntityTreeError),
}

impl From<&PreludeError> for EntitySynTreeError {
    fn from(_e: &PreludeError) -> Self {
        todo!()
    }
}

impl From<PreludeError> for EntitySynTreeError {
    fn from(e: PreludeError) -> Self {
        EntitySynTreeError::Derived(e.into())
    }
}

impl From<EntityTreeBundleError> for EntitySynTreeError {
    fn from(e: EntityTreeBundleError) -> Self {
        EntitySynTreeError::Derived(e.into())
    }
}

impl From<&EntityTreeBundleError> for EntitySynTreeError {
    fn from(_e: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<VfsError> for EntitySynTreeError {
    fn from(e: VfsError) -> Self {
        EntitySynTreeError::Derived(e.into())
    }
}

// impl From<&EntityTreeError> for EntityTreeError {
//     fn from(e: &EntityTreeError) -> Self {
//         todo!("e = {:?}", e)
//     }
// }

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
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
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
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

pub type EntitySynTreeResult<T> = Result<T, EntitySynTreeError>;
pub type EntityTreeResultRef<'a, T> = Result<T, &'a EntitySynTreeError>;
