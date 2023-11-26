mod internal;
#[cfg(test)]
mod tests;

use self::internal::MonoLinkTimeInternal;
#[cfg(test)]
use self::tests::*;
use husky_linkage::linkage::Linkage;
use husky_task::link::{IsLinkageImpl, IsLinktime};
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct MonoLinkTime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    internal: std::sync::RwLock<MonoLinkTimeInternal<LinkageImpl>>,
}

impl<LinkageImpl> IsLinktime for MonoLinkTime<LinkageImpl>
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
            internal: std::sync::RwLock::new(MonoLinkTimeInternal::new(target_path, db)),
        }
    }
}
