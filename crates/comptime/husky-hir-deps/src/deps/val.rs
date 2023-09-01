use crate::*;
use husky_val::Val;

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct HirValDeps {}

impl HasDeps for Val {
    type Deps = HirValDeps;

    fn deps(self, db: &dyn HirDepsDb) -> Self::Deps {
        todo!()
    }
}
