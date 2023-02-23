#![feature(trait_upcasting)]
mod collector;
mod db;
mod defn;
mod error;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use defn::*;
pub use error::*;
pub use sheet::*;

use collector::*;
use husky_decl::*;
use husky_entity_path::*;
use husky_entity_tree::*;
use husky_expr::ExprIdx;
use husky_expr::*;
use husky_vfs::{ModulePath, VfsResult};
use salsa::DbWithJar;

#[salsa::jar(db = DefnDb)]
pub struct DefnJar(
    // type
    EnumTypeDefn,
    enum_ty_defn,
    UnitStructTypeDefn,
    unit_struct_ty_defn,
    TupleStructTypeDefn,
    tuple_struct_ty_defn,
    RegularStructTypeDefn,
    regular_struct_ty_defn,
    RecordTypeDefn,
    record_ty_defn,
    InductiveTypeDefn,
    inductive_ty_defn,
    StructureTypeDefn,
    structure_ty_defn,
    AlienTypeDefn,
    alien_ty_defn,
    UnionTypeDefn,
    union_ty_defn,
    // form
    ValueDefn,
    // value_defn,
    FeatureDefn,
    feature_defn,
    FunctionDefn,
    function_defn,
    MorphismDefn,
    // morphism_defn,
    TypeAliasDefn,
    // type_alias_defn,
    // trait
    TraitDefn,
    trai_defn,
    // enum variant,
    UnitVariantDefn,
    unit_variant_defn,
    TupleVariantDefn,
    tuple_variant_defn,
    PropsVariantDefn,
    props_variant_defn,
    // type item
    TypeAssociatedFunctionDefn,
    ty_associated_function_defn,
    TypeMethodDefn,
    ty_method_defn,
    TypeAssociatedTypeDefn,
    ty_associated_ty_defn,
    TypeAssociatedValueDefn,
    ty_associated_value_defn,
    TypeMemoDefn,
    ty_memo_defn,
    // trait item
    TraitAssociatedFunctionDefn,
    trai_associated_function_defn,
    TraitMethodDefn,
    trai_method_defn,
    TraitAssociatedTypeDefn,
    trai_associated_ty_defn,
    TraitAssociatedValueDefn,
    trai_associated_value_defn,
    // type as trait item
    TypeAsTraitAssociatedFunctionDefn,
    ty_as_trai_associated_function_defn,
    TypeAsTraitMethodDefn,
    ty_as_trai_method_defn,
    TypeAsTraitAssociatedTypeDefn,
    ty_as_trai_associated_ty_defn,
    TypeAsTraitAssociatedValueDefn,
    ty_as_trai_associated_value_defn,
);

#[test]
fn defn_sheet_works() {
    use husky_vfs::VfsTestUtils;
    use tests::*;

    DB::default().vfs_expect_test_debug_with_db("defn_sheet", DefnDb::collect_defns);
}
