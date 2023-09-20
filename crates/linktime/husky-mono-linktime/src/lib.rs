mod internal;

use self::internal::MonoLinkTimeInternal;
use husky_hir_deps::{HirDepsDb, HirLinkageDeps};
use husky_linkage_path::LinkagePath;
use husky_regular_value::RegularValue;
use husky_task::*;
use husky_vfs::CratePath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct MonoLinkTime<Db, Linkage>
where
    Db: HirDepsDb,
    Linkage: IsLinkage,
{
    internal: std::sync::RwLock<MonoLinkTimeInternal<Db, Linkage>>,
}

impl<Db, Linkage> IsLinkageTable for MonoLinkTime<Db, Linkage>
where
    Db: HirDepsDb,
    Linkage: IsLinkage,
{
    type ComptimeDb = Db;

    type Linkage = Linkage;

    fn get_linkage(&self, key: LinkagePath, db: &Db) -> Linkage {
        if let Some(linkage) = self.internal.read().expect("todo").get_linkage(key, db) {
            linkage
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linkage_with_reload(key, db)
        }
    }

    fn new_linkage_table(target_crate: CratePath, db: &Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(MonoLinkTimeInternal::new(target_crate, db)),
        }
    }
}
