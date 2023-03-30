#![feature(trait_upcasting)]
mod db;
mod error;
mod has_signature;
mod parameter;
mod signature;
mod term;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::error::*;
pub use self::has_signature::*;
pub use self::parameter::*;
pub use self::signature::*;
pub use self::term::*;

use husky_decl::*;
use husky_entity_path::*;
use husky_raw_term::*;
use husky_term_prelude::*;
use husky_word::*;

#[salsa::jar(db = SignatureDb)]
pub struct SignatureJar(
    SignatureRawTermApplication,
    SignatureRawTermCurry,
    signature_term_region,
    // type
    enum_ty_signature,
    EnumTypeSignature,
    unit_struct_ty_signature,
    UnitStructTypeSignature,
    tuple_struct_ty_signature,
    TupleStructTypeSignature,
    regular_struct_ty_signature,
    RegularStructTypeSignature,
    record_ty_signature,
    RecordTypeSignature,
    inductive_ty_signature,
    InductiveTypeSignature,
    structure_ty_signature,
    StructureTypeSignature,
    alien_ty_signature,
    ExternTypeSignature,
    union_ty_signature,
    UnionTypeSignature,
    // trait
    trai_signature_from_decl,
    TraitSignature,
    // form
    // form_signature,
    value_signature,
    ValueSignature,
    feature_signature,
    FeatureSignature,
    form_fn_signature,
    FormFnSignature,
    morphism_signature,
    MorphismSignature,
    type_alias_signature,
    TypeAliasSignature,
    // impl block
    // impl_block_signature_from_decl,
    ty_impl_block_signature,
    TypeImplSignature,
    trai_for_ty_impl_block_signature,
    TraitForTypeImplBlockSignature,
    // variant
    UnitVariantSignature,
    PropsVariantSignature,
    TupleVariantSignature,
    // associated items
    // associated_item_signature_from_decl,
    // type item
    ty_associated_fn_signature,
    TypeAssociatedFnSignature,
    ty_method_signature,
    TypeMethodSignature,
    ty_associated_ty_signature_from_decl,
    TypeAssociatedTypeSignature,
    ty_associated_value_signature,
    TypeAssociatedValueSignature,
    ty_memo_signature,
    TypeMemoSignature,
    // trait item
    trai_associated_form_fn_signature,
    TraitAssociatedFormFnSignature,
    trai_method_signature,
    TraitMethodSignature,
    trai_associated_ty_signature,
    TraitAssociatedTypeSignature,
    trai_associated_value_signature,
    TraitAssociatedValueSignature,
    // type as trait item
    trai_for_ty_associated_form_fn_signature,
    TraitForTypeAssociatedFnSignature,
    trai_for_ty_method_signature,
    TraitForTypeMethodSignature,
    trai_for_ty_associated_ty_signature,
    TraitForTypeAssociatedTypeSignature,
    trai_for_ty_associated_value_signature,
    TraitForTypeAssociatedValueSignature,
    // decr
    DeriveDecrSignature,
    derive_decr_signature,
);
