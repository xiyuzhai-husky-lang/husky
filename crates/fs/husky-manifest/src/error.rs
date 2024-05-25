use husky_corgi_config::CorgiConfigError;
use husky_manifest_ast::ManifestAstError;
use husky_vfs::{error::VfsError, PackagePath};
use thiserror::Error;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestError {
    #[error("{0}")]
    Original(#[from] OriginalManifestError),
    #[error("{0}")]
    Derived(#[from] DerivedManifestError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalManifestError {
    #[error("CyclicDependendencies")]
    CyclicDependendencies {
        package_path: PackagePath,
        cyclic_dependent_package_paths: Vec<PackagePath>,
    },
    #[error("NoLibOrMainForPackage")]
    NoLibOrMainForPackage,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedManifestError {
    #[error("DerivedManifest")]
    Manifest,
}

pub type ManifestResult<T> = Result<T, ManifestError>;
pub type ManifestResultRef<'a, T> = Result<T, &'a ManifestError>;

impl From<&ManifestError> for ManifestError {
    fn from(e: &ManifestError) -> Self {
        DerivedManifestError::Manifest.into()
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
