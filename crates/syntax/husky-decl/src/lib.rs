#![feature(trait_upcasting)]
mod collector;
mod db;
mod decl;
mod error;
mod parameter;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl::*;
pub use error::*;
pub use parameter::*;
pub use sheet::*;

use collector::*;
use husky_ast::AstIdx;
use husky_entity_path::*;
use husky_entity_tree::EntityTreeResult;
use husky_expr::*;
use husky_vfs::{ModulePath, VfsResult};

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    decl_sheet,
    // type
    EnumTypeDecl,
    UnitStructTypeDecl,
    TupleStructTypeDecl,
    PropsStructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    AlienTypeDecl,
    UnionTypeDecl,
    // form
    ValueDecl,
    FeatureDecl,
    FunctionDecl,
    MorphismDecl,
    TypeAliasDecl,
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
    // variant
    UnitVariantDecl,
    PropsVariantDecl,
    TupleVariantDecl,
);

#[salsa::tracked(jar = DeclJar, return_ref)]
fn decl_sheet(db: &dyn DeclDb, module_path: ModulePath) -> EntityTreeResult<DeclSheet> {
    Ok(DeclCollector::new(db, module_path)?.collect_all())
}

#[test]
fn decl_sheet_works() {
    use husky_vfs::VfsTestSupport;
    use tests::*;

    DB::expect_test_probable_modules_debug_result_with_db("decl_sheet", DeclDb::decl_sheet);
}
