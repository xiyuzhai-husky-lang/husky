use husky_corgi_config::CorgiConfigError;
use husky_manifest_ast::ManifestAstError;
use husky_vfs::error::VfsError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestError {}

pub type ManifestResult<T> = Result<T, ManifestError>;
pub type ManifestResultRef<'a, T> = Result<T, &'a ManifestError>;

impl From<&ManifestError> for ManifestError {
    fn from(_value: &ManifestError) -> Self {
        todo!()
    }
}

impl From<&CorgiConfigError> for ManifestError {
    fn from(_value: &CorgiConfigError) -> Self {
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
