use husky_manifest_ast::ManifestAstError;
use husky_vfs::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ManifestError {}

pub type ManifestResult<T> = Result<T, ManifestError>;

impl From<&ManifestError> for ManifestError {
    fn from(_value: &ManifestError) -> Self {
        todo!()
    }
}

impl From<&VfsError> for ManifestError {
    fn from(_value: &VfsError) -> Self {
        todo!()
    }
}

impl From<VfsError> for ManifestError {
    fn from(_value: VfsError) -> Self {
        todo!()
    }
}

impl From<&ManifestAstError> for ManifestError {
    fn from(_value: &ManifestAstError) -> Self {
        todo!()
    }
}
