#![feature(trait_upcasting)]
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
#[cfg(test)]
use tests::*;

#[salsa::jar(db = DeclDb)]
pub struct DeclJar(
    // type
    ty_decl_aux,
    EnumTypeRawDecl,
    EnumTypeDecl,
    UnitStructTypeRawDecl,
    UnitStructTypeDecl,
    TupleStructTypeRawDecl,
    TupleStructTypeDecl,
    RegularStructTypeRawDecl,
    RegularStructTypeDecl,
    RecordTypeRawDecl,
    RecordTypeDecl,
    InductiveTypeRawDecl,
    InductiveTypeDecl,
    StructureTypeRawDecl,
    StructureTypeDecl,
    ExternTypeRawDecl,
    ExternTypeDecl,
    UnionTypeRawDecl,
    UnionTypeDecl,
    // trait
    trai_decl,
    TraitRawDecl,
    TraitDecl,
    // form
    fugitive_decl,
    ValRawDecl,
    ValDecl,
    FnRawDecl,
    FnDecl,
    GnRawDecl,
    GnDecl,
    TypeAliasRawDecl,
    TypeAliasDecl,
    // impl block
    ty_impl_block_decl_aux,
    TypeImplBlockRawDecl,
    TypeImplBlockDecl,
    trai_for_ty_impl_block_decl_aux,
    TraitForTypeImplBlockRawDecl,
    TraitForTypeImplBlockDecl,
    // variant
    ty_variant_decl,
    UnitVariantRawDecl,
    UnitVariantDecl,
    PropsVariantRawDecl,
    PropsVariantDecl,
    TupleVariantRawDecl,
    TupleVariantDecl,
    // associated items
    associated_item_decl,
    // type item
    ty_item_decls_map,
    TypeAssociatedFnRawDecl,
    TypeAssociatedFnDecl,
    TypeMethodFnRawDecl,
    TypeMethodFnDecl,
    TypeAssociatedTypeRawDecl,
    TypeAssociatedTypeDecl,
    TypeAssociatedValRawDecl,
    TypeAssociatedValDecl,
    TypeMemoizedFieldRawDecl,
    TypeMemoizedFieldDecl,
    // trait item
    TraitAssociatedFnRawDecl,
    TraitAssociatedFnDecl,
    TraitMethodFnRawDecl,
    TraitMethodFnDecl,
    TraitAssociatedTypeRawDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValRawDecl,
    TraitAssociatedValDecl,
    // type as trait item
    TraitForTypeAssociatedFnRawDecl,
    TraitForTypeAssociatedFnDecl,
    TraitForTypeMethodFnRawDecl,
    TraitForTypeMethodFnDecl,
    TraitForTypeAssociatedTypeRawDecl,
    TraitForTypeAssociatedTypeDecl,
    TraitForTypeAssociatedValRawDecl,
    TraitForTypeAssociatedValDecl,
);
