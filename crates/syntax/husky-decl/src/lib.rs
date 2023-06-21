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
use husky_expr::*;
use husky_token::*;
use husky_vfs::ModulePath;
use parsec::StreamParser;
use parser::*;
use smallvec::{SmallVec, ToSmallVec};
#[cfg(test)]
use tests::*;

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    // decl
    // - submodule
    SubmoduleNodeDecl,
    submodule_node_decl,
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
    RegularStructTypeNodeDecl,
    RegularStructTypeDecl,
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
    fugitive_node_decl,
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
    ty_impl_block_node_decl,
    TypeImplBlockDecl,
    ty_impl_block_decl,
    TraitForTypeImplBlockNodeDecl,
    trai_for_ty_impl_block_node_decl,
    TraitForTypeImplBlockDecl,
    trai_for_ty_impl_block_decl,
    // - variant
    ty_variant_decl,
    UnitVariantNodeDecl,
    UnitVariantDecl,
    PropsVariantNodeDecl,
    PropsVariantDecl,
    TupleVariantNodeDecl,
    TupleVariantDecl,
    // - associated items
    // - - type item
    ty_item_node_decl,
    ty_item_decl,
    ty_item_decls_map,
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
    // - - trait item
    TraitAssociatedFnNodeDecl,
    TraitAssociatedFnDecl,
    TraitMethodFnNodeDecl,
    TraitMethodFnDecl,
    TraitAssociatedTypeNodeDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValNodeDecl,
    TraitAssociatedValDecl,
    // - - trait for type item
    trai_for_ty_item_node_decl,
    trai_for_ty_item_decl,
    TraitForTypeAssociatedFnNodeDecl,
    TraitForTypeAssociatedFnDecl,
    TraitForTypeMethodFnNodeDecl,
    TraitForTypeMethodFnDecl,
    TraitForTypeAssociatedTypeNodeDecl,
    TraitForTypeAssociatedTypeDecl,
    TraitForTypeAssociatedValNodeDecl,
    TraitForTypeAssociatedValDecl,
    // sheet
    NodeDeclSheet,
    node_decl_sheet,
    DeclSheet,
    decl_sheet,
);
