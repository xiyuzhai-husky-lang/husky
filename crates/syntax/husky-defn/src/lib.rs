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
    ty_node_defn,
    ty_defn,
    EnumTypeNodeDefn,
    EnumTypeDefn,
    UnitStructTypeNodeDefn,
    UnitStructTypeDefn,
    TupleStructTypeNodeDefn,
    TupleStructTypeDefn,
    RegularStructTypeNodeDefn,
    RegularStructTypeDefn,
    RecordTypeNodeDefn,
    RecordTypeDefn,
    InductiveTypeNodeDefn,
    InductiveTypeDefn,
    StructureTypeNodeDefn,
    StructureTypeDefn,
    ExternTypeNodeDefn,
    ExternTypeDefn,
    UnionTypeNodeDefn,
    UnionTypeDefn,
    // fugitive
    fugitive_defn,
    ValNodeDefn,
    ValDefn,
    val_defn,
    FnNodeDefn,
    FnDefn,
    fn_defn,
    GnNodeDefn,
    GnDefn,
    gn_defn,
    // morphism_defn,
    TypeAliasNodeDefn,
    TypeAliasDefn,
    // type_alias_defn,
    // trait
    TraitNodeDefn,
    trai_node_defn,
    TraitDefn,
    trai_defn,
    // enum variant,
    UnitVariantNodeDefn,
    UnitVariantDefn,
    unit_variant_defn,
    TupleVariantNodeDefn,
    TupleVariantDefn,
    tuple_variant_defn,
    PropsVariantNodeDefn,
    PropsVariantDefn,
    props_variant_defn,
    // type item
    ty_item_defn,
    TypeAssociatedFnNodeDefn,
    TypeAssociatedFnDefn,
    ty_associated_fn_defn,
    TypeMethodFnNodeDefn,
    TypeMethodFnDefn,
    ty_method_fn_defn,
    TypeAssociatedTypeNodeDefn,
    TypeAssociatedTypeDefn,
    ty_associated_ty_defn,
    TypeAssociatedValNodeDefn,
    TypeAssociatedValDefn,
    ty_associated_val_defn,
    TypeMemoizedFieldNodeDefn,
    TypeMemoizedFieldDefn,
    ty_memo_defn,
    // trait item
    trai_item_defn,
    TraitAssociatedFnNodeDefn,
    TraitAssociatedFnDefn,
    trai_associated_fn_defn,
    TraitMethodFnNodeDefn,
    TraitMethodFnDefn,
    trai_method_defn,
    TraitAssociatedTypeNodeDefn,
    TraitAssociatedTypeDefn,
    trai_associated_ty_defn,
    TraitAssociatedValNodeDefn,
    TraitAssociatedValDefn,
    trai_associated_val_defn,
    // trait for type item
    trai_for_ty_item_defn,
    TraitForTypeAssociatedFnNodeDefn,
    TraitForTypeAssociatedFnDefn,
    trai_for_ty_associated_fn_defn,
    TraitForTypeMethodFnNodeDefn,
    TraitForTypeMethodFnDefn,
    trai_for_ty_method_defn,
    TraitForTypeAssociatedTypeNodeDefn,
    TraitForTypeAssociatedTypeDefn,
    trai_for_ty_associated_ty_defn,
    TraitForTypeAssociatedValNodeDefn,
    TraitForTypeAssociatedValDefn,
    trai_for_ty_associated_val_defn,
    // sheet
    module_defns,
);
