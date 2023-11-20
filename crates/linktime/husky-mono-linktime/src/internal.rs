mod linkage_storage;
mod mapgen;

use self::linkage_storage::MonoLinkageStorage;
use self::mapgen::generate_map;
use crate::*;
use husky_linkage_path::root::LinkageDeps;
use husky_rust_transpilation::db::RustTranspilationDb;
use husky_vfs::{linktime_target_path::LinktimeTargetPath, PackagePath};

pub struct MonoLinkTimeInternal<ComptimeDb, Linkage>
where
    ComptimeDb: LinkagePathDb + RustTranspilationDb,
    Linkage: IsLinkage,
{
    target_path: LinktimeTargetPath,
    linkage_storage: MonoLinkageStorage,
    map: HashMap<LinkagePath, (LinkageDeps, Linkage)>,
    _marker: PhantomData<ComptimeDb>,
}

impl<ComptimeDb, Linkage: IsLinkage> MonoLinkTimeInternal<ComptimeDb, Linkage>
where
    ComptimeDb: LinkagePathDb + RustTranspilationDb,
    Linkage: IsLinkage,
{
    pub(crate) fn new(target_path: LinktimeTargetPath, db: &ComptimeDb) -> Self {
        let linkage_storage = MonoLinkageStorage::generate(target_path, db);
        let map = generate_map(target_path, &linkage_storage, db);
        Self {
            target_path,
            linkage_storage,
            map,
            _marker: PhantomData,
        }
    }

    pub(crate) fn get_linkage(&self, key: LinkagePath, db: &ComptimeDb) -> Option<Linkage> {
        let (deps, linkage) = self.map.get(&key).copied().expect("todo");
        (deps == key.deps(db)).then_some(linkage)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linkage_with_reload(&mut self, key: LinkagePath, db: &ComptimeDb) -> Linkage {
        let (deps, linkage) = self.map.get(&key).copied().expect("should be some");
        if deps == key.deps(db) {
            return linkage;
        }
        self.reload(db);
        self.map.get(&key).copied().expect("should be some").1
    }

    fn reload(&mut self, db: &ComptimeDb) {
        self.linkage_storage = MonoLinkageStorage::generate(self.target_path, db);
        self.map = generate_map(self.target_path, &self.linkage_storage, db)
    }
}
