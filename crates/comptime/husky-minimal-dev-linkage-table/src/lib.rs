mod key;
mod linkage;

use self::key::*;
use self::linkage::*;
use husky_hir_deps::HirLinkageDeps;
use husky_task::*;
use std::{collections::HashMap, panic::RefUnwindSafe};

pub struct MinimalDevLinkageTable<Task: IsTask> {
    library_storage: DevLibraryStorage<Task>,
    map: HashMap<MinimalDevLinkageKey, (HirLinkageDeps, MinimalDevLinkage)>,
}

impl<Task: IsTask> RefUnwindSafe for MinimalDevLinkageTable<Task> {}

impl<Task: IsTask> IsLinkageTable for MinimalDevLinkageTable<Task> {
    type LinkageKey = MinimalDevLinkageKey;

    type Linkage = MinimalDevLinkage;

    fn get_linkage(&self, key: Self::LinkageKey) -> Option<(HirLinkageDeps, Self::Linkage)> {
        self.map.get(&key).copied()
    }
}
