mod internal;

use self::internal::BootLinkTimeInternal;
use husky_linkage::linkage::Linkage;
use husky_linkage_impl::IsLinkageImpl;
use husky_task::linktime::IsLinktime;
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
pub struct BootLinkTime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    internal: std::sync::RwLock<BootLinkTimeInternal<LinkageImpl>>,
}

impl<LinkageImpl> Default for BootLinkTime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    fn default() -> Self {
        Self {
            internal: Default::default(),
        }
    }
}

// ad hoc
unsafe impl<LinkageImpl> Send for BootLinkTime<LinkageImpl> where LinkageImpl: IsLinkageImpl {}

impl<LinkageImpl> IsLinktime for BootLinkTime<LinkageImpl>
where
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
