use crate::*;
use husky_linkage_table::{LinkageTable, ResolveLinkage};
use place::SingleAssignPlace;
use upcast::{Upcast, UpcastMut};

impl fmt::Debug for AnalysisHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("husky-compilerompileTime").finish()
    }
}

impl salsa::Database for AnalysisHost {}

impl salsa::ParallelDatabase for AnalysisHost {
    fn snapshot(&self) -> salsa::Snapshot<AnalysisHost> {
        salsa::Snapshot::new(AnalysisHost {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}
