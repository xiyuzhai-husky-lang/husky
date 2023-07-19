#![feature(trait_upcasting)]
mod db;
mod error;
pub mod helpers;
mod parameter;
mod signature;

pub use self::db::*;
pub use self::error::*;
pub use self::parameter::*;
pub use self::signature::*;

use self::parameter::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_ethereal_term::{instantiation::*, *};
use maybe_result::*;
use smallvec::*;

#[salsa::jar(db = EtherealSignatureDb)]
pub struct EtherealSignatureJar(
    // associated_items
    // - type items
    ty_item_ethereal_signature_templates_map,
    TypeMethodFnEtherealSignatureTemplate,
    TypeMethodFunctionEtherealSignatureTemplate,
    TypeMemoizedFieldEtherealSignatureTemplate,
    TypeAssociatedFnEtherealSignatureTemplate,
    ty_item_ethereal_signature_template,
    // - trait items
    TraitAssociatedFnEtherealSignatureTemplate,
    TraitMethodFnEtherealSignatureTemplate,
    // - trait for type
    TraitForTypeAssociatedTypeEtherealSignatureTemplate,
    TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated,
    trai_for_ty_item_ethereal_signature_template,
    // ty
    EnumEtherealSignatureTemplate,
    ExternEtherealSignatureTemplate,
    InductiveEtherealSignatureTemplate,
    RecordEtherealSignatureTemplate,
    PropsStructEtherealSignatureTemplate,
    StructureEtherealSignatureTemplate,
    TupleStructEtherealSignatureTemplate,
    UnionEtherealSignatureTemplate,
    UnitStructEtherealSignatureTemplate,
    ty_ethereal_signature_template,
    // ty variant
    EnumTupleTypeVariantEtherealSignatureTemplate,
    EnumPropsTypeVariantEtherealSignatureTemplate,
    EnumUnitTypeVariantEtherealSignatureTemplate,
    ty_variant_ethereal_signature_template,
    // impl block
    // - type
    TypeImplBlockEtherealSignatureTemplate,
    // - trait for type
    TraitForTypeImplBlockEtherealSignatureTemplate,
    TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
    trai_for_ty_impl_block_ethereal_signature_template,
    ty_impl_block_ethereal_signature_template,
    crate::helpers::trai_for_ty::ty_side_impl_block_signature_templates_map,
    crate::helpers::trai_for_ty::trai_side_derive_any_ethereal_signature_templates,
    trai_for_ty_impl_block_with_ty_instantiated_associated_output_ethereal_signature_template,
    trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template,
    // decr
    signature::ty_path_derive_decr_ethereal_signature_templates,
);
