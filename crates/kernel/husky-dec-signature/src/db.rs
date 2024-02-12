use husky_syn_expr::SynExprRegion;

use crate::*;

pub trait DecSignatureDb {
    fn syn_expr_dec_term_region(&self, syn_expr_region: SynExprRegion) -> &SynExprDecTermRegion;
}

impl DecSignatureDb for ::salsa::Db {
    fn syn_expr_dec_term_region(&self, syn_expr_region: SynExprRegion) -> &SynExprDecTermRegion {
        syn_expr_dec_term_region(self, syn_expr_region)
    }
}

// todo: remove unnecessary tracked functions
// replace them by associated functions
#[salsa::jar]
pub struct DecSignatureJar(
    syn_expr_dec_term_region,
    // type
    ty_dec_template,
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
    trai_syn_dec_template,
    // fugitive
    // fugitive_signature,
    fugitive_syn_dec_template,
    MajorValDecTemplate,
    MajorFnDecTemplate,
    MajorGnDecTemplate,
    TypeAliasDecTemplate,
    // impl block
    TypeImplBlockDecTemplate,
    ty_impl_block_syn_dec_template,
    TraitForTypeImplBlockDecTemplate,
    trai_for_ty_impl_block_syn_dec_template,
    // type variant
    ty_variant_dec_template,
    EnumUnitTypeVariantDecTemplate,
    EnumPropsVariantDecTemplate,
    EnumTupleVariantDecTemplate,
    // associated items
    // type item
    ty_item_syn_dec_template,
    TypeAssocFnDecTemplate,
    TypeMethodFnDecTemplate,
    TypeMethodFunctionDecTemplate,
    TypeAssocTypeDecTemplate,
    TypeAssocValDecTemplate,
    TypeMemoizedFieldDecTemplate,
    // trait item
    // trai_item_syn_dec_template,
    TraitAssocFnDecTemplate,
    TraitMethodFnDecTemplate,
    TraitAssocTypeDecTemplate,
    TraitAssocValDecTemplate,
    // type as trait item
    // trai_for_ty_item_syn_declarative_signature_from_decl,
    TraitForTypeAssocFnDecTemplate,
    TraitForTypeMethodFnDecTemplate,
    TraitForTypeAssocTypeDecTemplate,
    TraitForTypeAssocValDecTemplate,
    // attr
    // attr_dec_template,
    DeriveAttrDecTemplate,
    DeriveAttrShardDecTemplate,
);
