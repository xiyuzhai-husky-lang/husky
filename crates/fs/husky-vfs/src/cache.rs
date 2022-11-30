use crate::*;
use std::sync::Arc;

pub struct HuskyFileCache {
    data: Arc<DashMap<PathBufItd, SourceFile>>,
    kind: HuskyFileCacheKind,
}

pub enum HuskyFileCacheKind {
    Major,
    Snapshot,
}

impl Default for HuskyFileCache {
    fn default() -> Self {
        Self {
            data: Default::default(),
            kind: HuskyFileCacheKind::Major,
        }
    }
}

impl HuskyFileCache {
    pub fn snapshot(&self) -> Self {
        Self {
            data: self.data.clone(),
            kind: HuskyFileCacheKind::Snapshot,
        }
    }

    pub(crate) fn data(&self) -> &DashMap<PathBufItd, SourceFile> {
        &self.data
    }
}
