use crate::*;
use husky_fs_specs::FsSpecsError;
use husky_minimal_toml_utils::MinimalTomlError;
use husky_path_utils::PathUtilsError;
use std::path::PathBuf;

use thiserror::Error;

// todo: make this copyable
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = VfsDb)]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotExists(DiffPath),
    #[error("IO Error: ???")]
    IO {
        path: PathBuf,
        error_message: String,
    },
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
    #[error("minimal toml error")]
    MinimalToml(#[from] MinimalTomlError),
    #[error("package ident")]
    PackageIdent,
    #[error("derived {0}")]
    PathUtils(#[from] PathUtilsError),
    #[error("fs specs")]
    FsSpecs(#[from] FsSpecsError),
    #[error("FailToReadPackageNameFromManifest")]
    FailToReadPackageNameFromManifest,
}

impl From<&VfsError> for VfsError {
    fn from(value: &VfsError) -> Self {
        value.clone()
    }
}

impl From<&FsSpecsError> for VfsError {
    fn from(_value: &FsSpecsError) -> Self {
        todo!()
    }
}

pub type VfsResult<T> = Result<T, VfsError>;

impl VfsError {
    pub(crate) fn new_io_error(path: PathBuf, e: std::io::Error) -> VfsError {
        VfsError::IO {
            path,
            error_message: e.to_string(),
        }
    }
}
