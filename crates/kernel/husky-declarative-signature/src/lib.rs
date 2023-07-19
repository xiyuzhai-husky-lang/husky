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

use husky_coword::*;
use husky_decl::*;
use husky_declarative_term::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use smallvec::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

// todo: remove unnecessary tracked functions
// replace them by associated functions
#[salsa::jar(db = DeclarativeSignatureDb)]
pub struct DeclarativeSignatureJar(
    declarative_term_region,
    // type
    ty_declarative_signature_template,
    EnumDeclarativeSignatureTemplate,
    UnitStructDeclarativeSignatureTemplate,
    TupleStructDeclarativeSignatureTemplate,
    PropsStructDeclarativeSignatureTemplate,
    RecordDeclarativeSignatureTemplate,
    InductiveDeclarativeSignatureTemplate,
    StructureDeclarativeSignatureTemplate,
    ExternDeclarativeSignatureTemplate,
    UnionDeclarativeSignatureTemplate,
    // trait
    TraitDeclarativeSignatureTemplate,
    trai_declarative_signature_template,
    // fugitive
    // fugitive_signature,
    fugitive_declarative_signature_template,
    ValDeclarativeSignatureTemplate,
    val_declarative_signature_template,
    FnDeclarativeSignatureTemplate,
    fn_declarative_signature_template,
    GnDeclarativeSignatureTemplate,
    gn_declarative_signature,
    TypeAliasDeclarativeSignatureTemplate,
    type_alias_declarative_signature,
    // impl block
    // impl_block_signature_from_decl,
    TypeImplBlockDeclarativeSignatureTemplate,
    ty_impl_block_declarative_signature_template,
    TraitForTypeImplBlockDeclarativeSignatureTemplate,
    trai_for_ty_impl_block_declarative_signature_template,
    // type variant
    ty_variant_declarative_signature_template,
    EnumUnitTypeVariantDeclarativeSignatureTemplate,
    EnumPropsTypeVariantDeclarativeSignatureTemplate,
    EnumTupleTypeVariantDeclarativeSignatureTemplate,
    // associated items
    // associated_item_declarative_signature_from_decl,
    // type item
    ty_item_declarative_signature_template,
    ty_associated_fn_declarative_signature_template,
    TypeAssociatedFnDeclarativeSignatureTemplate,
    ty_method_fn_declarative_signature_template,
    TypeMethodFnDeclarativeSignatureTemplate,
    TypeMethodFunctionDeclarativeSignatureTemplate,
    ty_method_function_declarative_signature_template,
    TypeAssociatedTypeDeclarativeSignatureTemplate,
    ty_associated_ty_declarative_signature_template,
    TypeAssociatedValDeclarativeSignatureTemplate,
    ty_associated_val_declarative_signature_template,
    TypeMemoizedFieldDeclarativeSignatureTemplate,
    ty_memoized_field_declarative_signature_template,
    // trait item
    trai_item_declarative_signature_template,
    TraitAssociatedFnDeclarativeSignatureTemplate,
    trai_associated_form_fn_declarative_signature_template,
    TraitMethodFnDeclarativeSignatureTemplate,
    trai_method_fn_declarative_signature_template,
    TraitAssociatedTypeDeclarativeSignatureTemplate,
    trai_associated_ty_declarative_signature_template,
    TraitAssociatedValDeclarativeSignatureTemplate,
    trai_associated_val_declarative_signature_template,
    // type as trait item
    trai_for_ty_item_declarative_signature_from_decl,
    TraitForTypeAssociatedFnDeclarativeSignatureTemplate,
    trai_for_ty_associated_fn_declarative_signature_template,
    TraitForTypeMethodFnDeclarativeSignatureTemplate,
    trai_for_ty_method_fn_declarative_signature_template,
    TraitForTypeAssociatedTypeDeclarativeSignatureTemplate,
    trai_for_ty_associated_ty_declarative_signature_template,
    TraitForTypeAssociatedValDeclarativeSignatureTemplate,
    trai_for_ty_associated_val_declarative_signature_template,
    // decr
    DeriveDecrDeclarativeSignatureTemplate,
    derive_decr_declarative_signature_template,
    ty_path_derive_decr_declarative_signature_templates,
);
