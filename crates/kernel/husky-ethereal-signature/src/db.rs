use crate::*;

pub trait EtherealSignatureDb: salsa::DbWithJar<EtherealSignatureJar> + EtherealTermDb {}

impl<Db> EtherealSignatureDb for Db where Db: salsa::DbWithJar<EtherealSignatureJar> + EtherealTermDb
{}

#[salsa::jar(db = EtherealSignatureDb)]
pub struct EtherealSignatureJar(
    // associated_items
    // - type items
    ty_item_ethereal_signature_templates_map,
    TypeMethodFnEtherealSignatureTemplate,
    TypeMethodFunctionEtherealSignatureTemplate,
    TypeMemoizedFieldEtherealSignatureTemplate,
    TypeAssociatedFnEtherealSignatureTemplate,
    TypeAssociatedTypeEtherealSignatureTemplate,
    ty_item_ethereal_signature_template,
    // - trait items
    TraitAssociatedFnEtherealSignatureTemplate,
    TraitMethodFnEtherealSignatureTemplate,
    // - trait for type items
    TraitForTypeAssociatedFnEtherealSignatureTemplate,
    TraitForTypeAssociatedValEtherealSignatureTemplate,
    TraitForTypeAssociatedTypeEtherealSignatureTemplate,
    TraitForTypeAssociatedTypeEtherealSignatureTemplatePartiallyInstantiated,
    trai_for_ty_item_ethereal_signature_template,
    trai_for_ty_associated_ty_ethereal_signature_template_partially_instantiated_try_into_signature,
    TraitForTypeMethodFnEtherealSignatureTemplate,
    TraitForTypeMethodFnEtherealSignatureTemplatePartiallyInstantiated,
    trai_for_ty_method_fn_ethereal_signature_template_partially_instantiated_try_into_signature,
    // trai
    TraitEtherealSignatureTemplate,
    trai_ethereal_signature_template,
    // fugitive
    fugitive_ethereal_signature_template,
    FnFugitiveEtherealSignatureTemplate,
    GnFugitiveEtherealSignatureTemplate,
    TypeAliasFugitiveEtherealSignatureTemplate,
    ValFugitiveEtherealSignatureTemplate,
    // ty
    EnumTypeEtherealSignatureTemplate,
    ExternTypeEtherealSignatureTemplate,
    InductiveTypeEtherealSignatureTemplate,
    RecordTypeEtherealSignatureTemplate,
    PropsStructTypeEtherealSignatureTemplate,
    StructureTypeEtherealSignatureTemplate,
    TupleStructTypeEtherealSignatureTemplate,
    UnionTypeEtherealSignatureTemplate,
    UnitStructTypeEtherealSignatureTemplate,
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
    crate::helpers::trai_for_ty::trai_side_path_leading_trai_for_ty_impl_block_ethereal_signature_templates_map,
    crate::helpers::trai_for_ty::trai_side_derive_any_ethereal_signature_templates,
    trai_for_ty_impl_block_with_ty_instantiated_associated_output_ethereal_signature_template,
    trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template,
    // attr
    signature::ty_path_derive_attr_ethereal_signature_templates_map,
    attr_ethereal_signature_template,
    DeriveAttrEtherealSignatureTemplate,
    DeriveAttrShardEtherealSignatureTemplate
);
