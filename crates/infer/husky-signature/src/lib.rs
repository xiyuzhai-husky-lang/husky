#![feature(trait_upcasting)]
mod db;
mod error;
mod parameter;
mod signature;
mod term;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::error::*;
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
    function_signature,
    FunctionSignature,
    morphism_signature,
    MorphismSignature,
    type_alias_signature,
    TypeAliasSignature,
    // impl block
    // impl_block_signature_from_decl,
    ty_impl_block_signature,
    TypeImplSignature,
    ty_as_trai_im_signature,
    TypeAsTraitImplSignature,
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
    trai_associated_function_signature,
    TraitAssociatedFunctionSignature,
    trai_method_signature,
    TraitMethodSignature,
    trai_associated_ty_signature,
    TraitAssociatedTypeSignature,
    trai_associated_value_signature,
    TraitAssociatedValueSignature,
    // type as trait item
    ty_as_trai_associated_function_signature,
    TypeAsTraitAssociatedFnSignature,
    ty_as_trai_method_signature,
    TypeAsTraitMethodSignature,
    ty_as_trai_associated_ty_signature,
    TypeAsTraitAssociatedTypeSignature,
    ty_as_trai_associated_value_signature,
    TypeAsTraitAssociatedValueSignature,
);
