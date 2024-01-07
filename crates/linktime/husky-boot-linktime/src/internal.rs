mod libgen;
mod mapgen;

use crate::*;
use husky_linkage::version_stamp::LinkageVersionStamp;
use husky_vfs::CratePath;
use version_stamp::HasVersionStamp;

pub struct BootLinkTimeInternal<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    library_storage: BootLibraryStorage,
    map: HashMap<Linkage, (LinkageVersionStamp, LinkageImpl)>,
}

impl<LinkageImpl> Default for BootLinkTimeInternal<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    fn default() -> Self {
        Self {
            library_storage: todo!(),
            map: todo!(),
        }
    }
}

pub struct BootLibraryStorage {}

impl<LinkageImpl: IsLinkageImpl> BootLinkTimeInternal<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    pub(crate) fn new(_target_path: LinktimeTargetPath, _db: &::salsa::Db) -> Self {
        todo!()
        // let library_storage = generate_library(target_crate, db);
        // let map = generate_map(target_crate, &library_storage, db);
        // Self {
        //     target_crate: Some(target_crate),
        //     library_storage,
        //     map,
        //     _marker: PhantomData,
        // }
    }

    pub(crate) fn get_linkage(&self, linkage: Linkage, db: &::salsa::Db) -> Option<LinkageImpl> {
        let (version_stamp, linkage_impl) = self.map.get(&linkage).copied().expect("todo");
        (version_stamp == linkage.version_stamp(db)).then_some(linkage_impl)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linkage_with_reload(
        &mut self,
        linkage: Linkage,
        db: &::salsa::Db,
    ) -> LinkageImpl {
        let (deps, linkage_impl) = self.map.get(&linkage).copied().expect("todo");
        if deps == linkage.version_stamp(db) {
            return linkage_impl;
        }
        self.reload(db);
        todo!()
    }

    fn reload(&mut self, _db: &::salsa::Db) {
        todo!()
        // self.library_storage = generate_library(self.target_crate, db);
        // self.map = generate_map(self.target_crate, &self.library_storage, db)
    }
}
