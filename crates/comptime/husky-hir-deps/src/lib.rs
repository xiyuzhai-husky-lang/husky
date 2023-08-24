mod db;
mod deps;

pub use self::db::*;
pub use self::deps::*;

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct HirLinkageDeps {}
