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
    EnumTypeSynNodeDefn,
    EnumTypeSynDefn,
    UnitStructTypeSynNodeDefn,
    UnitStructTypeSynDefn,
    TupleStructTypeSynNodeDefn,
    TupleStructTypeSynDefn,
    PropsStructTypeSynNodeDefn,
    PropsStructTypeSynDefn,
    RecordTypeSynNodeDefn,
    RecordTypeSynDefn,
    InductiveTypeSynNodeDefn,
    InductiveTypeSynDefn,
    StructureTypeSynNodeDefn,
    StructureTypeSynDefn,
    ExternTypeSynNodeDefn,
    ExternTypeSynDefn,
    UnionTypeSynNodeDefn,
    UnionTypeSynDefn,
    // fugitive
    fugitive_syn_node_defn,
    fugitive_defn,
    ValSynNodeDefn,
    ValSynDefn,
    FnSynNodeDefn,
    FnDefn,
    GnNodeSynDefn,
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
    UnitVariantSynDefn,
    TupleVariantSynNodeDefn,
    TupleVariantSynDefn,
    PropsVariantSynNodeDefn,
    PropsVariantSynDefn,
    // type item
    ty_item_syn_node_defn,
    ty_item_defn,
    TypeAssociatedFnSynNodeDefn,
    TypeAssociatedFnSynDefn,
    TypeMethodFnSynNodeDefn,
    TypeMethodFnSynDefn,
    TypeAssociatedTypeSynNodeDefn,
    TypeAssociatedTypeSynDefn,
    TypeAssociatedValSynNodeDefn,
    TypeAssociatedValSynDefn,
    TypeMemoizedFieldSynNodeDefn,
    TypeMemoizedFieldSynDefn,
    // trait item
    trai_item_defn,
    TraitAssociatedFnSynNodeDefn,
    TraitAssociatedFnSynDefn,
    TraitMethodFnSynNodeDefn,
    TraitMethodFnSynDefn,
    TraitAssociatedTypeSynNodeDefn,
    TraitAssociatedTypeSynDefn,
    TraitAssociatedValSynNodeDefn,
    TraitAssociatedValSynDefn,
    // trait for type item
    trai_for_ty_item_syn_node_defn,
    trai_for_ty_item_defn,
    TraitForTypeAssociatedFnSynNodeDefn,
    TraitForTypeAssociatedFnSynDefn,
    TraitForTypeMethodFnSynNodeDefn,
    TraitForTypeMethodFnSynDefn,
    TraitForTypeAssociatedTypeSynNodeDefn,
    TraitForTypeAssociatedTypeSynDefn,
    TraitForTypeAssociatedValSynNodeDefn,
    TraitForTypeAssociatedValSynDefn,
    // sheet
    module_defns,
    module_node_defns,
);
