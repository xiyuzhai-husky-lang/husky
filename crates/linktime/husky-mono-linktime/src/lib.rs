/* ad hoc pub */
pub mod internal;
#[cfg(test)]
mod tests;

use self::internal::MonoLinkTimeInternal;
#[cfg(test)]
use self::tests::*;
use husky_linkage::linkage::Linkage;
use husky_task::linktime::IsLinktime;
use husky_task_prelude::IsLinkageImpl;
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
pub struct MonoLinkTime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    /* ad hoc pub*/ pub internal: std::sync::RwLock<MonoLinkTimeInternal<LinkageImpl>>,
}

impl<LinkageImpl> IsLinktime for MonoLinkTime<LinkageImpl>
where
    LinkageImpl: IsLinkageImpl,
{
    type LinkageImpl = LinkageImpl;

    fn linkage_impl(&self, key: Linkage, db: &::salsa::Db) -> LinkageImpl {
        if let Some(linkage) = self
            .internal
            .read()
            .expect("todo")
            .get_linkage_impl(key, db)
        {
            linkage
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linkage_impl_with_reload(key, db)
        }
    }

    fn new_linktime(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(MonoLinkTimeInternal::new(target_path, db)),
        }
    }
}
