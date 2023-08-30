mod internal;

use self::internal::MonoLinkageTableInternal;
use husky_hir_deps::{HirDepsDb, HirLinkageDeps, HirLinkageKey};
use husky_regular_value::__RegularValue;
use husky_task::*;
use husky_vfs::CratePath;
use std::{
    collections::HashMap,
    panic::{RefUnwindSafe, UnwindSafe},
};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct MonoLinkageTable<Linkage: IsLinkage> {
    internal: std::sync::RwLock<MonoLinkageTableInternal<Linkage>>,
}

impl<Linkage: IsLinkage> MonoLinkageTable<Linkage> {
    pub fn new(target_crate: CratePath, db: &dyn HirDepsDb) -> Self {
        Self {
            internal: std::sync::RwLock::new(MonoLinkageTableInternal::new(target_crate, db)),
        }
    }
}

impl<Linkage: IsLinkage> IsLinkageTable for MonoLinkageTable<Linkage> {
    type Linkage = Linkage;

    fn get_linkage(&self, key: HirLinkageKey, db: &dyn HirDepsDb) -> Linkage {
        if let Some(linkage) = self.internal.read().expect("todo").get_linkage(key, db) {
            linkage
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linkage_with_reload(key, db)
        }
    }
}
