use crate::*;
use husky_source_path::SourcePath;
use std::sync::Arc;

pub struct SourcePathMap {
    data: DashMap<SourcePath, SourceFile>,
}

pub enum HuskyFileCacheKind {
    Major,
    Snapshot,
}

impl Default for SourcePathMap {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl SourcePathMap {
    pub(crate) fn data(&self) -> &DashMap<SourcePath, SourceFile> {
        &self.data
    }
}
