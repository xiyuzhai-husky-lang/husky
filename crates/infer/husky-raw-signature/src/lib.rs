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
use husky_term::*;
use husky_word::*;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = RawSignatureDb)]
pub struct RawSignatureJar(
    RawSignatureTermApplication,
    RawSignatureTermCurry,
    raw_signature_term_region,
    // type
    enum_ty_raw_signature,
    EnumTypeRawSignature,
    unit_struct_ty_raw_signature,
    UnitStructTypeRawSignature,
    tuple_struct_ty_raw_signature,
    TupleStructTypeRawSignature,
    regular_struct_ty_raw_signature,
    RegularStructTypeRawSignature,
    record_ty_raw_signature,
    RecordTypeRawSignature,
    inductive_ty_raw_signature,
    InductiveTypeRawSignature,
    structure_ty_raw_signature,
    StructureTypeRawSignature,
    alien_ty_raw_signature,
    ExternTypeRawSignature,
    union_ty_raw_signature,
    UnionTypeRawSignature,
    // trait
    trai_raw_signature,
    TraitRawSignature,
    // form
    // form_raw_signature,
    value_raw_signature,
    ValueRawSignature,
    feature_raw_signature,
    FeatureRawSignature,
    function_raw_signature,
    FunctionRawSignature,
    morphism_raw_signature,
    MorphismRawSignature,
    type_alias_raw_signature,
    TypeAliasRawSignature,
    // impl block
    // im_raw_signature,
    ty_im_raw_signature,
    TypeImplRawSignature,
    ty_as_trai_im_raw_signature,
    TypeAsTraitImplRawSignature,
    // variant
    UnitVariantRawSignature,
    PropsVariantRawSignature,
    TupleVariantRawSignature,
    // associated items
    // associated_item_raw_signature,
    // type item
    ty_associated_function_raw_signature,
    TypeAssociatedFunctionRawSignature,
    ty_method_raw_signature,
    TypeMethodRawSignature,
    ty_associated_ty_raw_signature,
    TypeAssociatedTypeRawSignature,
    ty_associated_value_raw_signature,
    TypeAssociatedValueRawSignature,
    ty_memo_raw_signature,
    TypeMemoRawSignature,
    // trait item
    trai_associated_function_raw_signature,
    TraitAssociatedFunctionRawSignature,
    trai_method_raw_signature,
    TraitMethodRawSignature,
    trai_associated_ty_raw_signature,
    TraitAssociatedTypeRawSignature,
    trai_associated_value_raw_signature,
    TraitAssociatedValueRawSignature,
    // type as trait item
    ty_as_trai_associated_function_raw_signature,
    TypeAsTraitAssociatedFunctionRawSignature,
    ty_as_trai_method_raw_signature,
    TypeAsTraitMethodRawSignature,
    ty_as_trai_associated_ty_raw_signature,
    TypeAsTraitAssociatedTypeRawSignature,
    ty_as_trai_associated_value_raw_signature,
    TypeAsTraitAssociatedValueRawSignature,
);
