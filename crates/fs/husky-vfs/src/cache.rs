use crate::*;
use husky_absolute_path::AbsolutePath;
use std::sync::Arc;

#[derive(Default)]
pub struct VfsCache {
    files: DashMap<PathBuf, File>,
}

pub enum HuskyFileCacheKind {
    Major,
    Snapshot,
}

impl VfsCache {
    pub(crate) fn files(&self) -> &DashMap<PathBuf, File> {
        &self.files
    }
}
