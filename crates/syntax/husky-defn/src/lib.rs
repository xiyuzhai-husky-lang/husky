#![feature(trait_upcasting)]
mod collector;
mod db;
mod defn;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use defn::*;
use husky_entity_tree::EntityTreeResult;
pub use sheet::*;

use collector::*;
use husky_decl::*;
use husky_entity_path::*;
use husky_expr::*;
use husky_expr::{ExprIdx, ExprSheet};
use husky_vfs::{ModulePath, VfsResult};
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
    AlienTypeDefn,
    UnionTypeDefn,
    // form
    ValueDefn,
    FeatureDefn,
    FunctionDefn,
    MorphismDefn,
    TypeAliasDefn,
    // trait
    TraitDefn,
    // type item
    TypeAssociatedFunctionDefn,
    TypeMethodDefn,
    TypeAssociatedTypeDefn,
    TypeAssociatedValueDefn,
    TypeMemoDefn,
    // trait item
    TraitAssociatedFunctionDefn,
    TraitMethodDefn,
    TraitAssociatedTypeDefn,
    TraitAssociatedValueDefn,
);

#[salsa::tracked(jar = DefnJar, return_ref)]
fn defn_sheet(db: &dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<DefnSheet> {
    Ok(DefnCollector::new(db, module_path)?.collect_all())
}
