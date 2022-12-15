use crate::*;
use husky_absolute_path::AbsolutePath;
use husky_source_path::SourcePath;
use std::sync::Arc;

#[derive(Default)]
pub struct VfsCache {
    files: DashMap<AbsolutePath, File>,
}

pub enum HuskyFileCacheKind {
    Major,
    Snapshot,
}

impl VfsCache {
    pub(crate) fn files(&self) -> &DashMap<AbsolutePath, File> {
        &self.files
    }
}
