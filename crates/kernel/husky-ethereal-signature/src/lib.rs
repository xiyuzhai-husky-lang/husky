#![feature(trait_upcasting)]
mod db;
mod error;
mod parameter;
mod signature;

pub use self::db::*;
pub use self::error::*;
pub use self::parameter::*;
pub use self::signature::*;

use self::parameter::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_ethereal_term::{instantiator::*, *};
use maybe_result::*;
use smallvec::*;

#[salsa::jar(db = EtherealSignatureDb)]
pub struct EtherealSignatureJar(
    // type items
    ty_item_ethereal_signature_templates_map,
    TypeMethodFnEtherealSignatureTemplate,
    TypeMethodFunctionEtherealSignatureTemplate,
    TypeMemoizedFieldEtherealSignatureTemplate,
    TypeAssociatedFnEtherealSignatureTemplate,
    ty_item_ethereal_signature_template,
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
    TraitForTypeImplBlockEtherealSignatureTemplateWithTypeInstantiated,
    trai_for_ty_impl_block_ethereal_signature_template,
    ty_impl_block_ethereal_signature_template,
    ty_side_impl_block_signature_templates_map,
);
