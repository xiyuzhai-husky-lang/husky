#![feature(iter_advance_by)]
#![feature(trait_upcasting)]
#![feature(let_chains)]
mod db;
mod decl;
mod error;
mod parameter;
mod parser;
mod sheet;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::decl::*;
pub use self::error::*;
pub use self::parameter::*;
pub use self::sheet::*;

use derive_getters::Getters;
use either::*;
use husky_ast::*;
use husky_entity_path::*;
use husky_entity_tree::*;
use husky_syn_expr::*;
use husky_token::*;
use husky_vfs::ModulePath;
use parsec::StreamParser;
use parser::*;
use smallvec::{SmallVec, ToSmallVec};
#[cfg(test)]
use tests::*;

#[salsa::jar(db = DeclDb)]
pub struct SynDeclJar(
    // decl
    // - submodule
    SubmoduleSynNodeDecl,
    submodule_syn_node_decl,
    SubmoduleSynDecl,
    submodule_decl,
    // - type
    ty_node_decl,
    ty_decl,
    EnumTypeSynNodeDecl,
    EnumTypeSynDecl,
    UnitStructTypeSynNodeDecl,
    UnitStructTypeSynDecl,
    TupleStructTypeSynNodeDecl,
    TupleStructTypeSynDecl,
    PropsStructTypeSynNodeDecl,
    PropsStructTypeSynDecl,
    RecordTypeSynNodeDecl,
    RecordTypeSynDecl,
    InductiveTypeSynNodeDecl,
    InductiveTypeSynDecl,
    StructureTypeSynNodeDecl,
    StructureTypeSynDecl,
    ExternTypeSynNodeDecl,
    ExternTypeSynDecl,
    UnionTypeSynNodeDecl,
    UnionTypeSynDecl,
    // - trait
    TraitSynNodeDecl,
    trai_syn_node_decl,
    TraitSynDecl,
    trai_syn_decl,
    // - form
    fugitive_syn_node_decl,
    fugitive_syn_decl,
    ValSynNodeDecl,
    ValSynDecl,
    FnSynNodeDecl,
    FnSynDecl,
    GnSynNodeDecl,
    GnSynDecl,
    TypeAliasSynNodeDecl,
    TypeAliasSynDecl,
    // - impl block
    TypeImplBlockSynNodeDecl,
    ty_impl_block_syn_node_decl,
    TypeImplBlockSynDecl,
    ty_impl_block_syn_decl,
    TraitForTypeImplBlockSynNodeDecl,
    trai_for_ty_impl_block_syn_node_decl,
    TraitForTypeImplBlockSynDecl,
    trai_for_ty_impl_block_syn_decl,
    IllFormedImplBlockSynNodeDecl,
    ill_formed_impl_block_syn_node_decl,
    // - variant
    ty_variant_syn_node_decl,
    ty_variant_syn_decl,
    UnitTypeVariantSynNodeDecl,
    UnitTypeVariantSynDecl,
    PropsTypeVariantSynNodeDecl,
    PropsTypeVariantSynDecl,
    TupleTypeVariantSynNodeDecl,
    TupleTypeVariantSynDecl,
    // - associated items
    // -- type item
    ty_item_syn_node_decl,
    ty_item_syn_decl,
    TypeAssociatedFnSynNodeDecl,
    TypeAssociatedFnSynDecl,
    TypeMethodFnSynNodeDecl,
    TypeMethodFnSynDecl,
    TypeAssociatedTypeSynNodeDecl,
    TypeAssociatedTypeSynDecl,
    TypeAssociatedValSynNodeDecl,
    TypeAssociatedValSynDecl,
    TypeMemoizedFieldSynNodeDecl,
    TypeMemoizedFieldSynDecl,
    // -- trait item
    TraitAssociatedFnSynNodeDecl,
    TraitAssociatedFnSynDecl,
    TraitMethodFnSynNodeDecl,
    TraitMethodFnSynDecl,
    TraitAssociatedTypeSynNodeDecl,
    TraitAssociatedTypeSynDecl,
    TraitAssociatedValSynNodeDecl,
    TraitAssociatedValSynDecl,
    // -- trait for type item
    trai_for_ty_item_syn_node_decl,
    trai_for_ty_item_syn_decl,
    TraitForTypeAssociatedFnSynNodeDecl,
    TraitForTypeAssociatedFnSynDecl,
    TraitForTypeMethodFnSynNodeDecl,
    TraitForTypeMethodFnSynDecl,
    TraitForTypeAssociatedTypeSynNodeDecl,
    TraitForTypeAssociatedTypeSynDecl,
    TraitForTypeAssociatedValSynNodeDecl,
    TraitForTypeAssociatedValSynDecl,
    // -- ill formed item
    IllFormedItemSynNodeDecl,
    // sheet
    SynNodeDeclSheet,
    syn_node_decl_sheet,
    SynDeclSheet,
    syn_decl_sheet,
);
