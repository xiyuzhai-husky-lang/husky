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
    SubmoduleNodeDecl,
    submodule_syn_node_decl,
    SubmoduleDecl,
    submodule_decl,
    // - type
    ty_node_decl,
    ty_decl,
    EnumTypeNodeDecl,
    EnumTypeDecl,
    UnitStructTypeNodeDecl,
    UnitStructTypeDecl,
    TupleStructTypeNodeDecl,
    TupleStructTypeDecl,
    PropsStructTypeNodeDecl,
    PropsStructTypeDecl,
    RecordTypeNodeDecl,
    RecordTypeDecl,
    InductiveTypeNodeDecl,
    InductiveTypeDecl,
    StructureTypeNodeDecl,
    StructureTypeDecl,
    ExternTypeNodeDecl,
    ExternTypeDecl,
    UnionTypeNodeDecl,
    UnionTypeDecl,
    // - trait
    TraitNodeDecl,
    trai_node_decl,
    TraitDecl,
    trai_decl,
    // - form
    fugitive_syn_node_decl,
    fugitive_decl,
    ValNodeDecl,
    ValDecl,
    FnNodeDecl,
    FnDecl,
    GnNodeDecl,
    GnDecl,
    TypeAliasNodeDecl,
    TypeAliasDecl,
    // - impl block
    TypeImplBlockNodeDecl,
    ty_impl_block_syn_node_decl,
    TypeImplBlockDecl,
    ty_impl_block_decl,
    TraitForTypeImplBlockNodeDecl,
    trai_for_ty_impl_block_syn_node_decl,
    TraitForTypeImplBlockDecl,
    trai_for_ty_impl_block_decl,
    IllFormedImplBlockNodeDecl,
    ill_formed_impl_block_syn_node_decl,
    // - variant
    ty_variant_node_decl,
    ty_variant_decl,
    UnitTypeVariantNodeDecl,
    UnitTypeVariantDecl,
    PropsTypeVariantNodeDecl,
    PropsTypeVariantDecl,
    TupleTypeVariantNodeDecl,
    TupleTypeVariantDecl,
    // - associated items
    // -- type item
    ty_item_syn_node_decl,
    ty_item_decl,
    TypeAssociatedFnNodeDecl,
    TypeAssociatedFnDecl,
    TypeMethodFnNodeDecl,
    TypeMethodFnDecl,
    TypeAssociatedTypeNodeDecl,
    TypeAssociatedTypeDecl,
    TypeAssociatedValNodeDecl,
    TypeAssociatedValDecl,
    TypeMemoizedFieldNodeDecl,
    TypeMemoizedFieldDecl,
    // -- trait item
    TraitAssociatedFnNodeDecl,
    TraitAssociatedFnDecl,
    TraitMethodFnNodeDecl,
    TraitMethodFnDecl,
    TraitAssociatedTypeNodeDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValNodeDecl,
    TraitAssociatedValDecl,
    // -- trait for type item
    trai_for_ty_item_syn_node_decl,
    trai_for_ty_item_decl,
    TraitForTypeAssociatedFnNodeDecl,
    TraitForTypeAssociatedFnDecl,
    TraitForTypeMethodFnNodeDecl,
    TraitForTypeMethodFnDecl,
    TraitForTypeAssociatedTypeNodeDecl,
    TraitForTypeAssociatedTypeDecl,
    TraitForTypeAssociatedValNodeDecl,
    TraitForTypeAssociatedValDecl,
    // -- ill formed item
    IllFormedItemNodeDecl,
    // sheet
    NodeDeclSheet,
    node_decl_sheet,
    DeclSheet,
    decl_sheet,
);
