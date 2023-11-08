use crate::*;
use husky_linkage_path::LinkagePath;

impl HasDeps for LinkagePath {
    type Deps = LinkageDeps;

    fn deps(self, db: &dyn HirDepsDb) -> Self::Deps {
        todo!()
    }
}

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct LinkageDeps {}
