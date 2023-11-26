mod internal;
#[cfg(test)]
mod tests;

use self::internal::MonoLinkTimeInternal;
#[cfg(test)]
use self::tests::*;
use husky_linkage::{db::LinkageDb, linkage::Linkage};
use husky_rust_transpilation::db::RustTranspilationDb;
use husky_task::link::{IsLinkageImpl, IsLinktime};
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct MonoLinkTime<Db, LinkageImpl>
where
    Db: RustTranspilationDb,
    LinkageImpl: IsLinkageImpl,
{
    internal: std::sync::RwLock<MonoLinkTimeInternal<Db, LinkageImpl>>,
}

impl<Db, LinkageImpl> IsLinktime<Db> for MonoLinkTime<Db, LinkageImpl>
where
    Db: RustTranspilationDb,
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
