mod key;
mod linkage;

use self::key::*;
use self::linkage::*;
use husky_hir_deps::{HirDepsDb, HirLinkageDeps, HirLinkageKey};
use husky_regular_value::__RegularValue;
use husky_task::*;
use std::{
    collections::HashMap,
    panic::{RefUnwindSafe, UnwindSafe},
};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct MonoLinkageTable<Linkage: UnwindSafe + RefUnwindSafe + Copy> {
    library_storage: MonoLibraryStorage,
    map: HashMap<HirLinkageKey, (HirLinkageDeps, Linkage)>,
}

pub struct MonoLibraryStorage {}

impl<Linkage: UnwindSafe + RefUnwindSafe + Copy> RefUnwindSafe for MonoLinkageTable<Linkage> {}

impl<Linkage: UnwindSafe + RefUnwindSafe + Copy> IsLinkageTable for MonoLinkageTable<Linkage> {
    type Linkage = Linkage;

    fn get_linkage(&self, key: HirLinkageKey, db: &dyn HirDepsDb) -> Linkage {
        let (deps, linkage) = self.map.get(&key).copied().expect("todo");
        if deps != key.deps(db) {
            todo!("reload")
        } else {
            linkage
        }
    }
}
