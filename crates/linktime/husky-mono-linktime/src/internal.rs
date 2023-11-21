mod linkage_storage;
mod mapgen;

use self::linkage_storage::MonoLinkageStorage;
use self::mapgen::generate_map;
use crate::*;
use husky_linkage::version_stamp::LinkageVersionStamp;
use husky_rust_transpilation::db::RustTranspilationDb;
use husky_vfs::linktime_target_path::LinktimeTargetPath;

pub struct MonoLinkTimeInternal<ComptimeDb, LinkageImpl>
where
    ComptimeDb: LinkageDb + RustTranspilationDb,
    LinkageImpl: IsLinkageImpl,
{
    target_path: LinktimeTargetPath,
    linkage_storage: MonoLinkageStorage,
    map: HashMap<Linkage, (LinkageVersionStamp, LinkageImpl)>,
    _marker: PhantomData<ComptimeDb>,
}

impl<ComptimeDb, LinkageImpl: IsLinkageImpl> MonoLinkTimeInternal<ComptimeDb, LinkageImpl>
where
    ComptimeDb: LinkageDb + RustTranspilationDb,
    LinkageImpl: IsLinkageImpl,
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

    pub(crate) fn get_linkage(&self, linkage: Linkage, db: &ComptimeDb) -> Option<LinkageImpl> {
        let (deps, linkage_impl) = self.map.get(&linkage).copied().expect("todo");
        (deps == linkage.version_stamp(db)).then_some(linkage_impl)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linkage_with_reload(&mut self, key: Linkage, db: &ComptimeDb) -> LinkageImpl {
        let (deps, linkage) = self.map.get(&key).copied().expect("should be some");
        if deps == key.version_stamp(db) {
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
