#![feature(trait_upcasting)]
mod collector;
mod db;
mod defn;
mod error;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::defn::*;
pub use self::error::*;

use self::collector::*;
use husky_ast::*;
use husky_decl::*;
use husky_entity_path::*;
use husky_entity_tree::*;
use husky_expr::ExprIdx;
use husky_expr::*;
use husky_vfs::ModulePath;
use salsa::DbWithJar;

#[salsa::jar(db = DefnDb)]
pub struct DefnJar(
    // type
    ty_defn,
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
    ExternTypeDefn,
    alien_ty_defn,
    UnionTypeDefn,
    union_ty_defn,
    // fugitive
    fugitive_defn,
    ValDefn,
    val_defn,
    FnDefn,
    fn_defn,
    GnDefn,
    gn_defn,
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
    ty_item_defn,
    TypeAssociatedFnDefn,
    ty_associated_fn_defn,
    TypeMethodFnDefn,
    ty_method_fn_defn,
    TypeAssociatedTypeDefn,
    ty_associated_ty_defn,
    TypeAssociatedValDefn,
    ty_associated_val_defn,
    TypeMemoizedFieldDefn,
    ty_memo_defn,
    // trait item
    trai_item_defn,
    TraitAssociatedFnDefn,
    trai_associated_fn_defn,
    TraitMethodFnDefn,
    trai_method_defn,
    TraitAssociatedTypeDefn,
    trai_associated_ty_defn,
    TraitAssociatedValDefn,
    trai_associated_val_defn,
    // trait for type item
    trai_for_ty_item_defn,
    TraitForTypeAssociatedFnDefn,
    trai_for_ty_associated_fn_defn,
    TraitForTypeMethodFnDefn,
    trai_for_ty_method_defn,
    TraitForTypeAssociatedTypeDefn,
    trai_for_ty_associated_ty_defn,
    TraitForTypeAssociatedValDefn,
    trai_for_ty_associated_val_defn,
    // sheet
    module_defns,
);
