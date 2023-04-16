#![feature(trait_upcasting)]
mod db;
mod engine;
mod error;
mod has_signature;
mod parameter;
mod region;
mod signature;
#[cfg(test)]
mod tests;

pub use self::db::*;
pub use self::engine::*;
pub use self::error::*;
pub use self::has_signature::*;
pub use self::parameter::*;
pub use self::region::*;
pub use self::signature::*;

use husky_decl::*;
use husky_declarative_term::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use husky_word::*;

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
    var_signature,
    VarSignature,
    fn_signature,
    FnSignature,
    gn_signature,
    GnSignature,
    type_alias_signature,
    TypeAliasSignature,
    // impl block
    // impl_block_signature_from_decl,
    ty_impl_block_signature,
    TypeImplBlockSignature,
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
