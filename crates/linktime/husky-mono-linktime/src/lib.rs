mod internal;
#[cfg(test)]
mod tests;

use self::internal::MonoLinktimeInternal;
#[cfg(test)]
use self::tests::*;
use husky_devsoul::linktime::IsLinktime;
use husky_devsoul_interface::{IsDevRuntimeDyn, IsLinketImpl};
use husky_linket::linket::Linket;
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;
use std::collections::HashMap;

// this will transpile everything compilable to Rust
pub struct MonoLinktime<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    /* ad hoc pub*/ pub internal: std::sync::RwLock<MonoLinktimeInternal<LinketImpl>>,
}

impl<LinketImpl> IsLinktime for MonoLinktime<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    type LinketImpl = LinketImpl;

    fn linket_impl(&self, key: Linket, db: &::salsa::Db) -> LinketImpl {
        if let Some(linket) = self.internal.read().expect("todo").get_linket_impl(key, db) {
            linket
        } else {
            self.internal
                .write()
                .expect("todo")
                .get_linket_impl_with_reload(key, db)
        }
    }

    fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        Self {
            internal: std::sync::RwLock::new(MonoLinktimeInternal::new(target_path, db)),
        }
    }

    fn init(&self, runtime: &'static dyn IsDevRuntimeDyn<LinketImpl>) {
        let mut internal = self.internal.write().unwrap();
        internal.init(runtime)
    }
}
