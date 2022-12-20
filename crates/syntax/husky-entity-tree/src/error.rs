use husky_absolute_path::AbsolutePathError;
use husky_entity_path::EntityPathError;
use husky_package_path::PackagePathError;
use husky_toolchain_infer::ToolchainInferError;
use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeError {
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
    ToolchainInfer(#[from] ToolchainInferError),
}

impl From<&EntityTreeError> for EntityTreeError {
    fn from(value: &EntityTreeError) -> Self {
        EntityTreeError::DerivedSelf(Box::new(value.clone()))
    }
}

impl From<&VfsError> for EntityTreeError {
    fn from(e: &VfsError) -> Self {
        EntityTreeError::Vfs(e.clone())
    }
}

impl From<&ToolchainInferError> for EntityTreeError {
    fn from(e: &ToolchainInferError) -> Self {
        EntityTreeError::ToolchainInfer(e.clone())
    }
}

impl From<&EntityPathError> for EntityTreeError {
    fn from(value: &EntityPathError) -> Self {
        todo!()
    }
}

impl From<&PackagePathError> for EntityTreeError {
    fn from(value: &PackagePathError) -> Self {
        todo!()
    }
}

impl From<&AbsolutePathError> for EntityTreeError {
    fn from(value: &AbsolutePathError) -> Self {
        todo!()
    }
}

pub type EntityTreeResult<T> = Result<T, EntityTreeError>;
