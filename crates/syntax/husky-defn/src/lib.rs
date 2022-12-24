#![feature(trait_upcasting)]
mod collector;
mod db;
mod defn;
mod parser;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use defn::*;
pub use sheet::*;

use collector::*;
use husky_decl::*;
use husky_entity_path::*;
use husky_expr::{ExprArena, ExprIdx};
use husky_vfs::{ModulePath, VfsResult};
use parser::*;
use salsa::DbWithJar;

#[salsa::jar(db = DefnDb)]
pub struct DefnJar(
    defn_sheet,
    // type
    EnumTypeDefn,
    StructTypeDefn,
    RecordTypeDefn,
    InductiveTypeDefn,
    StructureTypeDefn,
    TypeAliasDefn,
    // form
    ConstantDefn,
    FeatureDefn,
    FunctionDefn,
    MethodDefn,
    MorphismDefn,
    // trait
    TraitDefn,
);

#[salsa::tracked(jar = DefnJar, return_ref)]
fn defn_sheet(db: &dyn DefnDb, module_path: ModulePath) -> VfsResult<DefnSheet> {
    Ok(DefnCollector::new(db, module_path)?.collect_all())
}
