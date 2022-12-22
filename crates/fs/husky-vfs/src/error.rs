use crate::*;
use husky_fs_specs::FsSpecsError;
use husky_minimal_toml_utils::MinimalTomlError;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VfsError {
    #[error("file {0:?} not found")]
    FileNotExists(PathBuf),
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

// impl From<std::io::Error> for VfsError {
//     fn from(value: std::io::Error) -> Self {
//         p!(value);
//         todo!()
//     }
// }

impl VfsError {
    pub(crate) fn new_io_error(path: PathBuf, e: std::io::Error) -> VfsError {
        VfsError::IO {
            path,
            error_message: e.to_string(),
        }
    }
}

impl salsa::DebugWithDb<dyn VfsDb + '_> for VfsError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn VfsDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for VfsError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn VfsDb, include_all_fields)
    }
}
