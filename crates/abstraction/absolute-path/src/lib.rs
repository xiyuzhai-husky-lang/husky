#![feature(absolute_path)]
mod error;

pub use error::*;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AbsolutePath {
    abs_path: Arc<PathBuf>,
}

impl AsRef<Path> for AbsolutePath {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl std::ops::Deref for AbsolutePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.abs_path
    }
}

impl std::borrow::Borrow<Path> for AbsolutePath {
    fn borrow(&self) -> &Path {
        self
    }
}

impl AbsolutePath {
    pub fn new(path: PathBuf) -> AbsolutePathResult<Self> {
        Ok(Self {
            abs_path: Arc::new(
                std::path::absolute(&path).map_err(|e| AbsolutePathError::IO {
                    path,
                    error_message: e.to_string(),
                })?,
            ),
        })
    }

    pub fn join(&self, path: impl AsRef<Path>) -> AbsolutePathResult<Self> {
        Self::new(self.abs_path.join(path))
    }
}
