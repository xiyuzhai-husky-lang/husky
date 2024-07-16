use self::region::SynExprDecTermRegion;
use crate::*;
use husky_syn_expr::region::SynExprRegion;

pub trait DecSignatureDb {
    fn syn_expr_dec_term_region(&self, syn_expr_region: SynExprRegion) -> &SynExprDecTermRegion;
}

impl DecSignatureDb for ::salsa::Db {
    #[track_caller]
    fn syn_expr_dec_term_region(&self, syn_expr_region: SynExprRegion) -> &SynExprDecTermRegion {
        crate::region::syn_expr_dec_term_region(self, syn_expr_region)
    }
}

// todo: remove unnecessary tracked functions
// replace them by associated functions
#[salsa::jar]
pub struct DecSignatureJar(
    crate::region::syn_expr_dec_term_region,
    // package
    crate::signature::package::PackageDecSignature,
    crate::signature::package::package_dec_signature,
    // crate
    crate::signature::crate_::lib::LibCrateDecSignature,
    crate::signature::crate_::task::TaskCrateDecSignature,
    crate::signature::crate_::requirements::RequirementsCrateDecSignature,
    crate::signature::crate_::main::MainCrateDecSignature,
    crate::signature::crate_::crate_dec_signature,
    // type
    crate::signature::major_item::ty::ty_dec_template,
    crate::signature::major_item::ty::r#enum::EnumDecTemplate,
    crate::signature::major_item::ty::unit_struct::UnitStructDecTemplate,
    crate::signature::major_item::ty::tuple_struct::TupleStructDecTemplate,
    crate::signature::major_item::ty::props_struct::PropsStructDecTemplate,
    crate::signature::major_item::ty::inductive::InductiveDecTemplate,
    crate::signature::major_item::ty::structure::StructureTypeDecTemplate,
    crate::signature::major_item::ty::r#extern::ExternDecTemplate,
    crate::signature::major_item::ty::union::UnionTypeDecTemplate,
    // trait
    crate::signature::major_item::trai::TraitDecTemplate,
    crate::signature::major_item::trai::trai_syn_dec_template,
    // form
    crate::signature::major_item::form::form_syn_dec_template,
    crate::signature::major_item::form::val::MajorValDecTemplate,
    crate::signature::major_item::form::compterm::MajorComptermDecTemplate,
    crate::signature::major_item::form::static_mut::MajorStaticMutDecTemplate,
    crate::signature::major_item::form::static_var::MajorStaticVarDecTemplate,
    crate::signature::major_item::form::function_ritchie::MajorFunctionRitchieDecTemplate,
    crate::signature::major_item::form::ty_alias::TypeAliasDecTemplate,
    crate::signature::major_item::form::ty_var::TypeVarDecTemplate,
    // impl block
    crate::signature::impl_block::ty_impl_block::TypeImplBlockDecTemplate,
    crate::signature::impl_block::ty_impl_block::ty_impl_block_syn_dec_template,
    crate::signature::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockDecTemplate,
    crate::signature::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_syn_dec_template,
    // type variant
    crate::signature::ty_variant::ty_variant_dec_template,
    crate::signature::ty_variant::enum_unit_ty_variant::EnumUnitTypeVariantDecTemplate,
    crate::signature::ty_variant::enum_props_ty_variant::EnumPropsVariantDecTemplate,
    crate::signature::ty_variant::enum_tuple_ty_variant::EnumTupleVariantDecTemplate,
    // associated items
    // type item
    crate::signature::assoc_item::ty_item::ty_item_syn_dec_template,
    crate::signature::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieDecTemplate,
    crate::signature::assoc_item::ty_item::method_ritchie::TypeMethodRitchieDecTemplate,
    crate::signature::assoc_item::ty_item::method_curry::TypeMethodCurryDecTemplate,
    crate::signature::assoc_item::ty_item::assoc_ty::TypeAssocTypeDecTemplate,
    crate::signature::assoc_item::ty_item::assoc_val::TypeAssocValDecTemplate,
    crate::signature::assoc_item::ty_item::memo_field::TypeMemoizedFieldDecTemplate,
    // trait item
    crate::signature::assoc_item::trai_item::trai_item_syn_dec_template,
    crate::signature::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieDecTemplate,
    crate::signature::assoc_item::trai_item::assoc_ty::TraitAssocTypeDecTemplate,
    crate::signature::assoc_item::trai_item::assoc_val::TraitAssocValDecTemplate,
    crate::signature::assoc_item::trai_item::assoc_static_mut::TraitAssocStaticMutDecTemplate,
    crate::signature::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarDecTemplate,
    crate::signature::assoc_item::trai_item::method_ritchie::TraitMethodRitchieDecTemplate,
    crate::signature::assoc_item::trai_item::memo_field::TraitMemoizedFieldDecTemplate,
    // type as trait item
    crate::signature::assoc_item::trai_for_ty_item::trai_for_ty_item_syn_declarative_signature_from_decl,
    crate::signature::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieDecTemplate,
    crate::signature::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieDecTemplate,
    crate::signature::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeDecTemplate,
    crate::signature::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValDecTemplate,
    // attr
    crate::signature::attr::attr_dec_template,
    crate::signature::attr::backprop::BackpropAttrDecTemplate,
    crate::signature::attr::deps::DepsAttrDecTemplate,
    crate::signature::attr::deps::DepsAttrShardDecTemplate,
    crate::signature::attr::derive::DeriveAttrDecTemplate,
    crate::signature::attr::derive::DeriveAttrShardDecTemplate,
    crate::signature::attr::projection::ProjectionAttrDecTemplate,
    crate::signature::attr::singleton::SingletonAttrDecTemplate,
    crate::signature::attr::task::TaskAttrDecTemplate,
);
