#[salsa::jar]
pub struct HirDeclJar(
    // attr
    crate::decl::attr::attr_hir_decl,
    // template
    crate::parameter::item_hir_template_parameter_stats,
    crate::decl::submodule_hir_decl,
    crate::decl::SubmoduleHirDecl,
    // associated_items
    // - type items
    // ty_item_hir_decl,
    crate::decl::TypeMethodFnHirDecl,
    crate::decl::TypeMemoizedFieldHirDecl,
    crate::decl::TypeAssociatedFnHirDecl,
    crate::decl::TypeAssociatedValHirDecl,
    crate::decl::TypeAssociatedTypeHirDecl,
    // - trait items
    crate::decl::TraitAssociatedFnHirDecl,
    crate::decl::TraitAssociatedTypeHirDecl,
    crate::decl::TraitAssociatedValHirDecl,
    crate::decl::TraitMethodFnHirDecl,
    // - trait for type
    crate::decl::trai_for_ty_item_hir_decl,
    crate::decl::TraitForTypeAssociatedFnHirDecl,
    crate::decl::TraitForTypeAssociatedTypeHirDecl,
    crate::decl::TraitForTypeAssociatedValHirDecl,
    crate::decl::TraitForTypeMethodFnHirDecl,
    // ty
    crate::decl::ty_hir_decl,
    crate::decl::EnumTypeHirDecl,
    crate::decl::ExternTypeHirDecl,
    crate::decl::RecordTypeHirDecl,
    crate::decl::PropsStructTypeHirDecl,
    crate::decl::TupleStructTypeHirDecl,
    crate::decl::UnionHirDecl,
    crate::decl::UnitStructHirDecl,
    // trai
    crate::decl::trai_hir_decl,
    crate::decl::TraitHirDecl,
    // fugitive
    crate::decl::fugitive_hir_decl,
    crate::decl::FunctionFnFugitiveHirDecl,
    crate::decl::FunctionGnFugitiveHirDecl,
    crate::decl::TypeAliasHirDecl,
    crate::decl::ValFugitiveHirDecl,
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
    crate::decl::DeriveAttrHirDecl,
    // helpers
    crate::helpers::enum_ty_has_only_unit_variants,
);
