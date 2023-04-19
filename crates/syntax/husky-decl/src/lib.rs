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
    EnumTypeDecl,
    UnitStructTypeDecl,
    TupleStructTypeDecl,
    RegularStructTypeDecl,
    RecordTypeDecl,
    InductiveTypeDecl,
    StructureTypeDecl,
    ExternTypeDecl,
    UnionTypeDecl,
    // trait
    trai_decl,
    TraitDecl,
    // form
    fugitive_decl,
    ValDecl,
    FnDecl,
    GnDecl,
    TypeAliasDecl,
    // impl block
    ty_impl_block_decl_aux,
    TypeImplBlockDecl,
    trai_for_ty_impl_block_decl_aux,
    TraitForTypeImplBlockDecl,
    // variant
    ty_variant_decl,
    UnitVariantDecl,
    PropsVariantDecl,
    TupleVariantDecl,
    // associated items
    associated_item_decl,
    // type item
    ty_item_decls_map,
    TypeAssociatedFnDecl,
    TypeMethodFnDecl,
    TypeAssociatedTypeDecl,
    TypeAssociatedValDecl,
    TypeMemoizedFieldDecl,
    // trait item
    TraitAssociatedFnDecl,
    TraitMethodFnDecl,
    TraitAssociatedTypeDecl,
    TraitAssociatedValDecl,
    // type as trait item
    TraitForTypeAssociatedFnDecl,
    TraitForTypeMethodFnDecl,
    TraitForTypeAssociatedTypeDecl,
    TraitForTypeAssociatedValDecl,
);
