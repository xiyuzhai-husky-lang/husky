#![feature(trait_upcasting)]
mod db;
mod error;
mod parameter;
mod signature;
mod term;
#[cfg(test)]
mod tests;

pub use db::*;
pub use error::*;
pub use parameter::*;
pub use signature::*;
pub use term::*;

use husky_decl::*;
use husky_entity_path::*;
use husky_term::*;
use husky_word::*;

#[cfg(test)]
use tests::*;

#[salsa::jar(db = SignatureDb)]
pub struct SignatureJar(
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
    AlienTypeSignature,
    union_ty_signature,
    UnionTypeSignature,
    // trait
    trai_signature,
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
    // impl_block_signature,
    ty_impl_block_signature,
    TypeImplBlockSignature,
    ty_as_trai_impl_block_signature,
    TypeAsTraitImplBlockSignature,
    // variant
    UnitVariantSignature,
    PropsVariantSignature,
    TupleVariantSignature,
    // associated items
    // associated_item_signature,
    // type item
    ty_associated_function_signature,
    TypeAssociatedFunctionSignature,
    ty_method_signature,
    TypeMethodSignature,
    ty_associated_ty_signature,
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
    TypeAsTraitAssociatedFunctionSignature,
    ty_as_trai_method_signature,
    TypeAsTraitMethodSignature,
    ty_as_trai_associated_ty_signature,
    TypeAsTraitAssociatedTypeSignature,
    ty_as_trai_associated_value_signature,
    TypeAsTraitAssociatedValueSignature,
);
