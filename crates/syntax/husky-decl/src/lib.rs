#![feature(trait_upcasting)]
mod db;
mod decl;
mod error;
mod parameter;
mod parse;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use decl::*;
pub use error::*;
pub use parameter::*;
pub use sheet::*;

use husky_ast::AstIdx;
use husky_entity_path::*;
use husky_entity_tree::EntityTreeResult;
use husky_expr::*;
use husky_vfs::{ModulePath, VfsResult};
use parse::*;

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    module_decl_sheet,
    // type
    type_decl,
    EnumTypeDecl,
    UnitStructTypeDecl,
    TupleStructTypeDecl,
    PropsStructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    AlienTypeDecl,
    UnionTypeDecl,
    // trait
    trait_decl,
    TraitDecl,
    // form
    form_decl,
    ValueDecl,
    FeatureDecl,
    FunctionDecl,
    MorphismDecl,
    TypeAliasDecl,
    // impl block
    impl_block_decl,
    TypeImplBlockDecl,
    TypeAsTraitImplBlockDecl,
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
fn module_decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntityTreeResult<DeclSheet> {
    DeclSheet::collect_from_module(db, path)
}

#[test]
fn decl_sheet_works() {
    use husky_vfs::VfsTestSupport;
    use tests::*;

    DB::expect_test_probable_modules_debug_result_with_db("decl_sheet", DeclDb::module_decl_sheet);
}
