mod internal;

use self::internal::BootLinkTimeInternal;
use husky_devsoul::linktime::IsLinktime;
use husky_devsoul_interface::IsLinketImpl;
use husky_linket::linket::Linket;
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;
use std::collections::HashMap;

// this will transpile everything compilable to Rust
pub struct BootLinkTime<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    internal: std::sync::RwLock<BootLinkTimeInternal<LinketImpl>>,
}

impl<LinketImpl> Default for BootLinkTime<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    fn default() -> Self {
        Self {
            internal: Default::default(),
        }
    }
}

// ad hoc
unsafe impl<LinketImpl> Send for BootLinkTime<LinketImpl> where LinketImpl: IsLinketImpl {}

impl<LinketImpl> IsLinktime for BootLinkTime<LinketImpl>
where
    LinketImpl: IsLinketImpl<Pedestal = ()>,
{
    type LinketImpl = LinketImpl;

    fn linket_impl(&self, key: Linket, db: &::salsa::Db) -> LinketImpl {
        if let Some(linket) = self.internal.read().expect("todo").get_linket(key, db) {
            linket
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linket_with_reload(key, db)
        }
    }

    fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(BootLinkTimeInternal::new(target_path, db)),
        }
    }

    fn init(&self, runtime: &dyn husky_devsoul_interface::IsDevRuntimeDyn<Self::LinketImpl>) {
        todo!()
    }
}
