mod collector;
mod db;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use sheet::*;

use collector::*;
use husky_vfs::{ModulePath, VfsResult};

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(Decl, decl_sheet);

#[salsa::tracked(jar = DeclJar)]
pub struct Decl {
    #[return_ref]
    data: DeclData,
}

#[salsa::tracked(jar = DeclJar, return_ref)]
fn decl_sheet(db: &dyn DeclDb, module_path: ModulePath) -> VfsResult<DeclSheet> {
    Ok(DeclCollector::new(db, module_path)?.collect_all())
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeclData {}
