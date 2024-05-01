use husky_syn_expr::SynExprRegion;

use crate::*;

pub trait DecSignatureDb {
    fn syn_expr_dec_term_region(&self, syn_expr_region: SynExprRegion) -> &SynExprDecTermRegion;
}

impl DecSignatureDb for ::salsa::Db {
    #[track_caller]
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
    EnumDecTemplate,
    UnitStructDecTemplate,
    TupleStructDecTemplate,
    PropsStructDecTemplate,
    InductiveTypeDecTemplate,
    StructureTypeDecTemplate,
    ExternTypeDecTemplate,
    UnionTypeDecTemplate,
    // trait
    TraitDecTemplate,
    trai_syn_dec_template,
    // form
    form_syn_dec_template,
    MajorValDecTemplate,
    crate::signature::major_item::form::r#const::MajorConstDecTemplate,
    MajorFunctionRitchieDecTemplate,
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
    TypeAssocRitchieDecTemplate,
    TypeMethodRitchieDecTemplate,
    TypeMethodCurryDecTemplate,
    TypeAssocTypeDecTemplate,
    TypeAssocValDecTemplate,
    TypeMemoizedFieldDecTemplate,
    // trait item
    trai_item_syn_dec_template,
    TraitAssocRitchieDecTemplate,
    TraitMethodRitchieDecTemplate,
    TraitAssocTypeDecTemplate,
    TraitAssocValDecTemplate,
    // type as trait item
    trai_for_ty_item_syn_declarative_signature_from_decl,
    TraitForTypeAssocRitchieDecTemplate,
    TraitForTypeMethodRitchieDecTemplate,
    TraitForTypeAssocTypeDecTemplate,
    TraitForTypeAssocValDecTemplate,
    // attr
    attr_dec_template,
    DeriveAttrDecTemplate,
    DeriveAttrShardDecTemplate,
);
