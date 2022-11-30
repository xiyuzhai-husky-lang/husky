use crate::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PhysicalPath {
    canonical_path: Arc<PathBuf>,
}

impl AsRef<Path> for PhysicalPath {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl std::ops::Deref for PhysicalPath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.canonical_path
    }
}

impl std::borrow::Borrow<Path> for PhysicalPath {
    fn borrow(&self) -> &Path {
        self
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PhysicalPathError {
    #[error("IO")]
    IO,
}

pub type PhysicalPathResult<T> = Result<T, PhysicalPathError>;

impl From<std::io::Error> for PhysicalPathError {
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}

impl PhysicalPath {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        Ok(Self {
            canonical_path: Arc::new(path.canonicalize()?),
        })
    }
}

#[salsa::tracked(jar = SourcePathJar, return_ref)]
pub(crate) fn physical_path(
    db: &dyn SourcePathDb,
    path: SourcePath,
) -> PhysicalPathResult<PhysicalPath> {
    todo!()
}
