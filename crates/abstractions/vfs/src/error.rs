use crate::*;
use husky_io_utils::error::IoError;
use husky_path_utils::PathUtilsError;
use maybe_result::MaybeResult;
use thiserror::Error;

// todo: make this copyable
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotExists(VirtualPath),
    #[error("IO Error: ???")]
    Io {
        path: PathBuf,
        error_message: String,
    },
    #[error("io error: {0}")]
    Io2(#[from] IoError),
    #[error("not source file")]
    NotSourceFile(PathBuf),
    #[error("fail to absolutize {path:?} due to IO `{error_message}")]
    FailToAbsolutize {
        path: PathBuf,
        error_message: String,
    },
    #[error("fail to diff")]
    FailToDiff,
    #[error("failed to resolve module path")]
    ModulePathResolveFailure,
    #[error("package ident")]
    PackageIdent,
    #[error("derived {0}")]
    PathUtils(#[from] PathUtilsError),
    #[error("FailToReadPackageNameFromManifest")]
    FailToReadPackageNameFromManifest,
}

impl From<&VfsError> for VfsError {
    fn from(value: &VfsError) -> Self {
        value.clone()
    }
}

pub type VfsResult<T> = Result<T, VfsError>;
pub type VfsMaybeResult<T> = MaybeResult<T, VfsError>;

impl VfsError {
    pub(crate) fn new_io_error(path: PathBuf, e: std::io::Error) -> VfsError {
        VfsError::Io {
            path,
            error_message: e.to_string(),
        }
    }
}
