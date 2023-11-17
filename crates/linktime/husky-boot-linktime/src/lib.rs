mod internal;

use self::internal::BootLinkTimeInternal;
use husky_linkage_path::{db::LinkagePathDb, LinkagePath};

use husky_task::link::{IsLinkage, IsLinktime};
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use std::{collections::HashMap, marker::PhantomData};

// this will transpile everything compilable to Rust
// then use rustc to obtain a single dylib
pub struct BootLinkTime<Db, Linkage>
where
    Db: LinkagePathDb,
    Linkage: IsLinkage,
{
    internal: std::sync::RwLock<BootLinkTimeInternal<Db, Linkage>>,
}

impl<Db, Linkage> Default for BootLinkTime<Db, Linkage>
where
    Db: LinkagePathDb,
    Linkage: IsLinkage,
{
    fn default() -> Self {
        Self {
            internal: Default::default(),
        }
    }
}

// ad hoc
unsafe impl<Db, Linkage> Send for BootLinkTime<Db, Linkage>
where
    Db: LinkagePathDb,
    Linkage: IsLinkage,
{
}

impl<Db, Linkage> IsLinktime<Db> for BootLinkTime<Db, Linkage>
where
    Db: LinkagePathDb,
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

    fn new_linktime(target_path: LinktimeTargetPath, db: &Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(BootLinkTimeInternal::new(target_path, db)),
        }
    }
}
