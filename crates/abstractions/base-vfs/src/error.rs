use crate::*;
use husky_io_utils::error::IoError;
use husky_path_utils::PathUtilsError;
use maybe_result::MaybeResult;
use thiserror::Error;

// todo: make this copyable
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub enum BaseVfsError {
    #[error("file {0:?} not found")]
    FileNotExists(VirtualPath),
    #[error("IO Error: ???")]
    Io {
        path: PathBuf,
        error_message: String,
    },
    #[error("io error: {0}")]
    Io2(#[from] IoError),
    #[error("fail to diff paths")]
    FailToDiffPaths,
    #[error("not source file")]
    NotSourceFile(PathBuf),
    #[error("fail to absolutize {path:?} due to IO `{error_message}")]
    FailToAbsolutize {
        path: PathBuf,
        error_message: String,
    },
    #[error("derived {0}")]
    PathUtils(#[from] PathUtilsError),
}

impl From<&BaseVfsError> for BaseVfsError {
    fn from(value: &BaseVfsError) -> Self {
        value.clone()
    }
}

pub type BaseVfsResult<T> = Result<T, BaseVfsError>;
pub type BaseVfsMaybeResult<T> = MaybeResult<T, BaseVfsError>;

impl BaseVfsError {
    pub(crate) fn new_io_error(path: PathBuf, e: std::io::Error) -> BaseVfsError {
        BaseVfsError::Io {
            path,
            error_message: e.to_string(),
        }
    }
}
