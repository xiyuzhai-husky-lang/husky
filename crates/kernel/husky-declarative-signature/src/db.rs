use husky_syn_expr::SynExprRegion;

use crate::*;

pub trait DeclarativeSignatureDb {
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion;
}

impl DeclarativeSignatureDb for ::salsa::Db {
    fn declarative_term_region(&self, syn_expr_region: SynExprRegion) -> &DeclarativeTermRegion {
        declarative_term_region(self, syn_expr_region)
    }
}

// todo: remove unnecessary tracked functions
// replace them by associated functions
#[salsa::jar]
pub struct DeclarativeSignatureJar(
    declarative_term_region,
    // type
    ty_declarative_signature_template,
    EnumTypeDecTemplate,
    UnitStructTypeDecTemplate,
    TupleStructTypeDecTemplate,
    PropsStructTypeDecTemplate,
    InductiveTypeDecTemplate,
    StructureTypeDecTemplate,
    ExternTypeDecTemplate,
    UnionTypeDecTemplate,
    // trait
    TraitDecTemplate,
    trai_syn_declarative_signature_template,
    // fugitive
    // fugitive_signature,
    // fugitive_syn_declarative_signature_template,
    MajorValDecTemplate,
    MajorFnDecTemplate,
    MajorGnDecTemplate,
    TypeAliasDecTemplate,
    // impl block
    // impl_block_signature_from_decl,
    TypeImplBlockDecTemplate,
    ty_impl_block_syn_declarative_signature_template,
    TraitForTypeImplBlockDecTemplate,
    trai_for_ty_impl_block_syn_declarative_signature_template,
    // type variant
    ty_variant_syn_declarative_signature_template,
    EnumUnitTypeVariantDecTemplate,
    EnumPropsVariantDecTemplate,
    EnumTupleVariantDecTemplate,
    // associated items
    // associated_item_syn_declarative_signature_from_decl,
    // type item
    // ty_item_syn_declarative_signature_template,
    TypeAssociatedFnDecTemplate,
    TypeMethodFnDecTemplate,
    TypeMethodFunctionDecTemplate,
    TypeAssociatedTypeDecTemplate,
    TypeAssociatedValDecTemplate,
    TypeMemoizedFieldDecTemplate,
    // trait item
    // trai_item_syn_declarative_signature_template,
    TraitAssociatedFnDecTemplate,
    TraitMethodFnDecTemplate,
    TraitAssociatedTypeDecTemplate,
    TraitAssociatedValDecTemplate,
    // type as trait item
    // trai_for_ty_item_syn_declarative_signature_from_decl,
    TraitForTypeAssociatedFnDecTemplate,
    TraitForTypeMethodFnDecTemplate,
    TraitForTypeAssociatedTypeDecTemplate,
    TraitForTypeAssociatedValDecTemplate,
    // attr
    // attr_declarative_signature_template,
    DeriveAttrDecTemplate,
    DeriveAttrShardDecTemplate,
);
