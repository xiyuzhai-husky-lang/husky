mod libgen;
mod mapgen;

use crate::*;
use husky_vfs::CratePath;

use self::libgen::generate_library;
use self::mapgen::generate_map;

pub struct MonoLinkageTableInternal<Linkage: IsLinkage> {
    target_crate: CratePath,
    library_storage: MonoLibraryStorage,
    map: HashMap<HirLinkageKey, (HirLinkageDeps, Linkage)>,
}

pub struct MonoLibraryStorage {}

impl<Linkage: IsLinkage> MonoLinkageTableInternal<Linkage> {
    pub(crate) fn new(target_crate: CratePath, db: &dyn HirDepsDb) -> Self {
        let library_storage = generate_library(target_crate, db);
        let map = generate_map(target_crate, &library_storage, db);
        Self {
            target_crate,
            library_storage,
            map,
        }
    }

    pub(crate) fn get_linkage(&self, key: HirLinkageKey, db: &dyn HirDepsDb) -> Option<Linkage> {
        let (deps, linkage) = self.map.get(&key).copied().expect("todo");
        (deps == key.deps(db)).then_some(linkage)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linkage_with_reload(
        &mut self,
        key: HirLinkageKey,
        db: &dyn HirDepsDb,
    ) -> Linkage {
        let (deps, linkage) = self.map.get(&key).copied().expect("todo");
        if deps == key.deps(db) {
            return linkage;
        }
        todo!("reload")
    }

    fn reload(&mut self, db: &dyn HirDepsDb) {
        self.library_storage = generate_library(self.target_crate, db);
        self.map = generate_map(self.target_crate, &self.library_storage, db)
    }
}
