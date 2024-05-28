#[salsa::jar]
pub struct HirDeclJar(
    // crate
    crate::decl::crate_::lib::LibCrateHirDecl,
    crate::decl::crate_::main::MainCrateHirDecl,
    crate::decl::crate_::task::TaskCrateHirDecl,
    // attr
    crate::decl::attr::attr_hir_decl,
    // template
    crate::parameter::item_hir_template_parameter_stats,
    crate::decl::submodule_hir_decl,
    crate::decl::SubmoduleHirDecl,
    // assoc_items
    // - type items
    // ty_item_hir_decl,
    crate::decl::assoc_item::ty_item::ty_item_hir_decl,
    crate::decl::TypeMethodRitchieHirDecl,
    crate::decl::TypeMemoFieldHirDecl,
    crate::decl::TypeAssocRitchieHirDecl,
    crate::decl::TypeAssocValHirDecl,
    crate::decl::TypeAssocTypeHirDecl,
    // - trait items
    crate::decl::TraitAssocRitchieHirDecl,
    crate::decl::TraitAssocTypeHirDecl,
    crate::decl::TraitAssocValHirDecl,
    crate::decl::TraitMethodFnHirDecl,
    // - trait for type
    crate::decl::trai_for_ty_item_hir_decl,
    crate::decl::TraitForTypeAssocRitchieHirDecl,
    crate::decl::TraitForTypeAssocTypeHirDecl,
    crate::decl::TraitForTypeAssocValHirDecl,
    crate::decl::TraitForTypeMethodRitchieHirDecl,
    // ty
    crate::decl::ty_hir_decl,
    crate::decl::EnumHirDecl,
    crate::decl::ExternTypeHirDecl,
    crate::decl::PropsStructHirDecl,
    crate::decl::TupleStructHirDecl,
    crate::decl::UnionHirDecl,
    crate::decl::UnitStructHirDecl,
    // trai
    crate::decl::trai_hir_decl,
    crate::decl::TraitHirDecl,
    // major form
    crate::decl::major_item::form::major_form_hir_decl,
    crate::decl::major_item::form::function_ritchie::MajorFunctionRitchieHirDecl,
    crate::decl::major_item::form::ty_alias::MajorTypeAliasHirDecl,
    crate::decl::major_item::form::val::MajorValHirDecl,
    crate::decl::major_item::form::compterm::MajorComptermHirDecl,
    crate::decl::major_item::form::r#static::MajorStaticHirDecl,
    // ty variant
    crate::decl::ty_variant_hir_decl,
    crate::decl::EnumTupleVariantHirDecl,
    crate::decl::EnumPropsVariantHirDecl,
    crate::decl::EnumUnitTypeVariantHirDecl,
    // impl block
    // - type
    crate::decl::ty_impl_block_hir_decl,
    crate::decl::TypeImplBlockHirDecl,
    // - trait for type
    crate::decl::trai_for_ty_impl_block_hir_decl,
    crate::decl::TraitForTypeImplBlockHirDecl,
    // attr
    crate::decl::attr::backprop::BackpropAttrHirDecl,
    crate::decl::attr::derive::DeriveAttrHirDecl,
    crate::decl::attr::affect::AffectAttrHirDecl,
    crate::decl::attr::marker::MarkerAttrHirDecl,
    crate::decl::attr::task::TaskAttrHirDecl,
    crate::decl::attr::test::TestAttrHirDecl,
    // helpers
    crate::helpers::enum_ty_has_only_unit_variants,
);
