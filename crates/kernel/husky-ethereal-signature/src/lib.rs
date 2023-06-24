#![feature(trait_upcasting)]
mod db;
mod error;
mod instantiator;
mod parameter;
mod signature;

pub use self::db::*;
pub use self::error::*;
pub use self::parameter::*;
pub use self::signature::*;

use self::parameter::*;
use husky_entity_path::*;
use husky_ethereal_term::*;
use husky_word::*;
use maybe_result::*;
use smallvec::*;

#[salsa::jar(db = EtherealSignatureDb)]
pub struct EtherealSignatureJar(
    // type items
    ty_item_ethereal_signature_templates_map,
    TypeAssociatedFnEtherealSignatureTemplate,
    ty_method_ethereal_signature_templates_map,
    TypeMethodFnEtherealSignatureTemplate,
    ty_method_fn_ethereal_signature_template,
    TypeMethodFunctionEtherealSignatureTemplate,
    ty_memoized_field_ethereal_signature_templates_map,
    TypeMemoizedFieldEtherealSignatureTemplate,
    ty_memoized_field_ethereal_signature_template,
    // ty
    EnumEtherealSignatureTemplate,
    enum_ethereal_signature_template,
    ExternEtherealSignatureTemplate,
    InductiveEtherealSignatureTemplate,
    RecordEtherealSignatureTemplate,
    RegularStructEtherealSignatureTemplate,
    StructureEtherealSignatureTemplate,
    TupleStructEtherealSignatureTemplate,
    UnionEtherealSignatureTemplate,
    UnitStructEtherealSignatureTemplate,
    // impl block
    TypeImplBlockEtherealSignatureTemplate,
    ty_impl_block_ethereal_signature_template,
);
