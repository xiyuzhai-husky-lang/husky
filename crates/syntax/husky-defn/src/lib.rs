mod collector;
mod db;
mod form;
mod parser;
mod sheet;
#[cfg(test)]
mod tests;
mod trai;
mod ty;

pub use db::*;
pub use form::*;
pub use sheet::*;
pub use trai::*;
pub use ty::*;

use collector::*;
use husky_decl::*;
use husky_entity_path::EntityPath;
use husky_expr::{ExprArena, ExprIdx};
use husky_vfs::{ModulePath, VfsResult};
use parser::*;
use salsa::DbWithJar;

#[salsa::jar(db = DefnDb)]
pub struct DefnJar(Defn, defn_sheet);

#[salsa::tracked(jar = DefnJar, return_ref)]
fn defn_sheet(db: &dyn DefnDb, module_path: ModulePath) -> VfsResult<DefnSheet> {
    Ok(DefnCollector::new(db, module_path)?.collect_all())
}

#[salsa::tracked(jar = DefnJar)]
pub struct Defn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub variant: DefnVariant,
    #[return_ref]
    pub arena: ExprArena,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DefnVariant {
    Submodule,
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
}
