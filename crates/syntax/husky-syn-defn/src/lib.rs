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
use husky_entity_path::*;
use husky_entity_tree::*;
use husky_syn_decl::*;
use husky_syn_expr::*;
use husky_vfs::ModulePath;
use salsa::DbWithJar;

#[salsa::jar(db = SynDefnDb)]
pub struct SynDefnJar(
    // type
    ty_node_defn,
    ty_defn,
    EnumTypeNodeDefn,
    EnumTypeDefn,
    UnitStructTypeNodeDefn,
    UnitStructTypeDefn,
    TupleStructTypeNodeDefn,
    TupleStructTypeDefn,
    PropsStructTypeNodeDefn,
    PropsStructTypeDefn,
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
    fugitive_node_defn,
    fugitive_defn,
    ValNodeDefn,
    ValDefn,
    FnNodeDefn,
    FnDefn,
    GnNodeDefn,
    GnDefn,
    // morphism_defn,
    TypeAliasNodeDefn,
    TypeAliasDefn,
    // type_alias_defn,
    // trait
    TraitSynNodeDefn,
    trai_node_defn,
    TraitDefn,
    trai_defn,
    // enum variant,
    UnitVariantSynNodeDefn,
    UnitVariantDefn,
    TupleVariantSynNodeDefn,
    TupleVariantDefn,
    PropsVariantSynNodeDefn,
    PropsVariantDefn,
    // type item
    ty_item_node_defn,
    ty_item_defn,
    TypeAssociatedFnNodeDefn,
    TypeAssociatedFnDefn,
    TypeMethodFnNodeDefn,
    TypeMethodFnDefn,
    TypeAssociatedTypeNodeDefn,
    TypeAssociatedTypeDefn,
    TypeAssociatedValNodeDefn,
    TypeAssociatedValDefn,
    TypeMemoizedFieldNodeDefn,
    TypeMemoizedFieldDefn,
    // trait item
    trai_item_defn,
    TraitAssociatedFnNodeDefn,
    TraitAssociatedFnDefn,
    TraitMethodFnNodeDefn,
    TraitMethodFnDefn,
    TraitAssociatedTypeNodeDefn,
    TraitAssociatedTypeDefn,
    TraitAssociatedValNodeDefn,
    TraitAssociatedValDefn,
    // trait for type item
    trai_for_ty_item_node_defn,
    trai_for_ty_item_defn,
    TraitForTypeAssociatedFnNodeDefn,
    TraitForTypeAssociatedFnDefn,
    TraitForTypeMethodFnNodeDefn,
    TraitForTypeMethodFnDefn,
    TraitForTypeAssociatedTypeNodeDefn,
    TraitForTypeAssociatedTypeDefn,
    TraitForTypeAssociatedValNodeDefn,
    TraitForTypeAssociatedValDefn,
    // sheet
    module_defns,
    module_node_defns,
);
