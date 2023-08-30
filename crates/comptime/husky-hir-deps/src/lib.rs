mod db;
mod deps;

pub use self::db::*;
pub use self::deps::*;

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct HirLinkageDeps {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirLinkageKey {
    FnCall {},
    MethodFnCall {},
    FieldAccess {},
}

impl HirLinkageKey {
    pub fn deps(self, db: &dyn HirDepsDb) -> HirLinkageDeps {
        todo!()
    }
}
