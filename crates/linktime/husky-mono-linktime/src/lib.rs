mod internal;
#[cfg(test)]
mod tests;

use self::internal::MonoLinkTimeInternal;
#[cfg(test)]
use self::tests::*;
use husky_linkage_path::{db::LinkagePathDb, LinkagePath};
use husky_rust_transpilation::db::RustTranspilationDb;
use husky_task::linkage::{IsLinkage, IsLinktime};
use husky_vfs::CratePath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct MonoLinkTime<Db, Linkage>
where
    Db: RustTranspilationDb,
    Linkage: IsLinkage,
{
    internal: std::sync::RwLock<MonoLinkTimeInternal<Db, Linkage>>,
}

impl<Db, Linkage> IsLinktime<Db> for MonoLinkTime<Db, Linkage>
where
    Db: RustTranspilationDb,
    Linkage: IsLinkage,
{
    type Linkage = Linkage;

    fn get_linkage(&self, key: LinkagePath, db: &Db) -> Linkage {
        if let Some(linkage) = self.internal.read().expect("todo").get_linkage(key, db) {
            linkage
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linkage_with_reload(key, db)
        }
    }

    fn new_linktime(target_crate: CratePath, db: &Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(MonoLinkTimeInternal::new(target_crate, db)),
        }
    }
}
