#![feature(trait_upcasting)]
mod collector;
mod db;
mod decl;
mod error;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl::*;
pub use error::*;
pub use sheet::*;

use collector::*;
use husky_entity_path::*;
use husky_expr::*;
use husky_vfs::{ModulePath, VfsResult};

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    decl_sheet,
    // type
    EnumTypeDecl,
    StructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    AliasTypeDecl,
    // form
    ConstantDecl,
    FeatureDecl,
    FunctionDecl,
    MorphismDecl,
    // trait
    TraitDecl,
    // type item
    TypeAssociatedFunctionDecl,
    TypeMethodDecl,
    TypeAssociatedTypeDecl,
    TypeAssociatedValueDecl,
    TypeMemoDecl,
    // trait item
    TraitAssociatedFunctionDecl,
    TraitMethodDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValueDecl,
);

#[salsa::tracked(jar = DeclJar, return_ref)]
fn decl_sheet(db: &dyn DeclDb, module_path: ModulePath) -> DeclResult<DeclSheet> {
    Ok(DeclCollector::new(db, module_path)?.collect_all())
}
