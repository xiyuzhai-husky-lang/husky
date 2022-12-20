#![feature(absolute_path)]

use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct AbsolutePath(PathBuf);

impl AbsolutePath {
    pub fn new(path: &Path) -> AbsolutePathResult<Self> {
        Ok(AbsolutePath(std::path::absolute(&path).map_err(|e| {
            AbsolutePathError::FailToAbsolutize {
                path: path.to_owned(),
                error_message: e.to_string(),
            }
        })?))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for AbsolutePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum AbsolutePathError {
    #[error("fail to absolutize {path:?} due to IO `{error_message}")]
    FailToAbsolutize {
        path: PathBuf,
        error_message: String,
    },
}

impl From<&AbsolutePathError> for AbsolutePathError {
    fn from(value: &AbsolutePathError) -> Self {
        todo!()
    }
}

pub type AbsolutePathResult<T> = Result<T, AbsolutePathError>;
