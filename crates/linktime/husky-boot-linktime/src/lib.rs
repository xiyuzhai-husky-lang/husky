mod internal;

use self::internal::BootLinkTimeInternal;
use husky_linkage::{db::LinkageDb, linkage::Linkage};

use husky_task::link::{IsLinkageImpl, IsLinktime};
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct BootLinkTime<Db, LinkageImpl>
where
    Db: LinkageDb,
    LinkageImpl: IsLinkageImpl,
{
    internal: std::sync::RwLock<BootLinkTimeInternal<Db, LinkageImpl>>,
}

impl<Db, LinkageImpl> Default for BootLinkTime<Db, LinkageImpl>
where
    Db: LinkageDb,
    LinkageImpl: IsLinkageImpl,
{
    fn default() -> Self {
        Self {
            internal: Default::default(),
        }
    }
}

// ad hoc
unsafe impl<Db, LinkageImpl> Send for BootLinkTime<Db, LinkageImpl>
where
    Db: LinkageDb,
    LinkageImpl: IsLinkageImpl,
{
}

impl<Db, LinkageImpl> IsLinktime<Db> for BootLinkTime<Db, LinkageImpl>
where
    Db: LinkageDb,
    LinkageImpl: IsLinkageImpl,
{
    type LinkageImpl = LinkageImpl;

    fn get_linkage(&self, key: Linkage, db: &::salsa::Db) -> LinkageImpl {
        if let Some(linkage) = self.internal.read().expect("todo").get_linkage(key, db) {
            linkage
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linkage_with_reload(key, db)
        }
    }

    fn new_linktime(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(BootLinkTimeInternal::new(target_path, db)),
        }
    }
}
