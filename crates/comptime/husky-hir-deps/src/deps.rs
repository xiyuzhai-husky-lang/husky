mod linkage;
mod val;

pub use self::linkage::*;
pub use self::val::*;

use crate::*;

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct Deps {}

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct DepPaths0 {}

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct DepPaths1 {}

#[salsa::interned(db = HirDepsDb, jar = HirDepsJar)]
pub struct DepPaths2 {}
