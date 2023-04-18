#![feature(trait_upcasting)]
mod db;
mod engine;
mod error;
mod parameter;
mod region;
mod signature;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::engine::*;
pub use self::error::*;
pub use self::parameter::*;
pub use self::region::*;
pub use self::signature::*;

use husky_decl::*;
use husky_declarative_term::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use husky_word::*;

#[salsa::jar(db = DeclarativeSignatureDb)]
pub struct DeclarativeSignatureJar(
    declarative_term_region,
    // type
    enum_ty_declarative_signature,
    EnumTypeDeclarativeSignature,
    unit_struct_ty_declarative_signature,
    UnitStructTypeDeclarativeSignature,
    tuple_struct_ty_declarative_signature,
    TupleStructTypeDeclarativeSignature,
    regular_struct_ty_declarative_signature,
    RegularStructTypeDeclarativeSignature,
    record_ty_declarative_signature,
    RecordTypeDeclarativeSignature,
    inductive_ty_declarative_signature,
    InductiveTypeDeclarativeSignature,
    structure_ty_declarative_signature,
    StructureTypeDeclarativeSignature,
    alien_ty_declarative_signature,
    ExternTypeDeclarativeSignature,
    union_ty_declarative_signature,
    UnionTypeDeclarativeSignature,
    // trait
    trai_declarative_signature_from_decl,
    TraitDeclarativeSignature,
    // form
    // form_signature,
    var_signature,
    ValDeclarativeSignature,
    fn_declarative_signature,
    FnDeclarativeSignature,
    gn_signature,
    GnSignature,
    type_alias_signature,
    TypeAliasSignature,
    // impl block
    // impl_block_signature_from_decl,
    ty_impl_block_declarative_signature,
    TypeImplBlockSignature,
    trai_for_ty_impl_block_declarative_signature,
    TraitForTypeImplBlockDeclarativeSignature,
    // variant
    UnitVariantDeclarativeSignature,
    PropsVariantDeclarativeSignature,
    TupleVariantDeclarativeSignature,
    // associated items
    // associated_item_declarative_signature_from_decl,
    // type item
    ty_associated_fn_declarative_signature,
    TypeAssociatedFnDeclarativeSignature,
    ty_method_signature,
    TypeMethodSignature,
    ty_associated_ty_declarative_signature_from_decl,
    TypeAssociatedTypeDeclarativeSignature,
    ty_associated_val_declarative_signature,
    TypeAssociatedValueSignature,
    ty_memo_signature,
    TypeMemoSignature,
    // trait item
    trai_associated_form_fn_declarative_signature,
    TraitAssociatedFormFnDeclarativeSignature,
    trai_method_signature,
    TraitMethodSignature,
    trai_associated_ty_declarative_signature,
    TraitAssociatedTypeDeclarativeSignature,
    trai_associated_val_declarative_signature,
    TraitAssociatedValueSignature,
    // type as trait item
    trai_for_ty_associated_fn_declarative_signature,
    TraitForTypeAssociatedFnDeclarativeSignature,
    trai_for_ty_method_fn_signature,
    TraitForTypeMethodFnDeclarativeSignature,
    trai_for_ty_associated_ty_declarative_signature,
    TraitForTypeAssociatedTypeDeclarativeSignature,
    trai_for_ty_associated_val_declarative_signature,
    TraitForTypeAssociatedValueSignature,
    // decr
    DeriveDecrDeclarativeSignature,
    derive_decr_declarative_signature,
);
