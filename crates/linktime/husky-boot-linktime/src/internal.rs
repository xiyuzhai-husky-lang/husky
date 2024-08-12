mod libgen;
mod mapgen;

use crate::*;
use husky_linket::version_stamp::LinketVersionStamp;
use husky_linket_impl::linket_impl::IsLinketImpl;
use husky_vfs::path::crate_path::CratePath;
use version_stamp::HasVersionStamp;

pub struct BootLinkTimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    library_storage: BootLibraryStorage,
    map: HashMap<Linket, (LinketVersionStamp, LinketImpl)>,
}

impl<LinketImpl> Default for BootLinkTimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    fn default() -> Self {
        Self {
            library_storage: todo!(),
            map: todo!(),
        }
    }
}

pub struct BootLibraryStorage {}

impl<LinketImpl: IsLinketImpl> BootLinkTimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
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

    pub(crate) fn get_linket(&self, linket: Linket, db: &::salsa::Db) -> Option<LinketImpl> {
        let (version_stamp, linket_impl) = self.map.get(&linket).copied().expect("todo");
        (version_stamp == linket.version_stamp(db)).then_some(linket_impl)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linket_with_reload(
        &mut self,
        linket: Linket,
        db: &::salsa::Db,
    ) -> LinketImpl {
        let (deps, linket_impl) = self.map.get(&linket).copied().expect("todo");
        if deps == linket.version_stamp(db) {
            return linket_impl;
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
