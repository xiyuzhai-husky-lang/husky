use crate::*;

use husky_vfs::ModulePath;

pub trait SynDeclDb {
    fn syn_node_decl_sheet(&self, module_path: ModulePath) -> SynNodeDeclSheet;

    fn syn_decl_sheet(&self, module_path: ModulePath) -> SynDeclSheet;
}

impl SynDeclDb for ::salsa::Db {
    fn syn_node_decl_sheet(&self, module_path: ModulePath) -> SynNodeDeclSheet {
        syn_node_decl_sheet(self, module_path)
    }

    fn syn_decl_sheet(&self, module_path: ModulePath) -> SynDeclSheet {
        syn_decl_sheet(self, module_path)
    }
}

#[salsa::jar]
pub struct SynDeclJar(
    // decl
    // - submodule
    crate::decl::submodule::SubmoduleSynNodeDecl,
    crate::decl::submodule::submodule_syn_node_decl,
    crate::decl::submodule::SubmoduleSynDecl,
    crate::decl::submodule::submodule_decl,
    // - type
    crate::decl::major_item::ty::ty_node_decl,
    crate::decl::major_item::ty::ty_decl,
    crate::decl::major_item::ty::EnumTypeSynNodeDecl,
    crate::decl::major_item::ty::EnumTypeSynDecl,
    crate::decl::major_item::ty::UnitStructTypeSynNodeDecl,
    crate::decl::major_item::ty::UnitStructTypeSynDecl,
    crate::decl::major_item::ty::TupleStructTypeSynNodeDecl,
    crate::decl::major_item::ty::TupleStructTypeSynDecl,
    crate::decl::major_item::ty::PropsStructTypeSynNodeDecl,
    crate::decl::major_item::ty::PropsStructTypeSynDecl,
    crate::decl::major_item::ty::InductiveTypeSynNodeDecl,
    crate::decl::major_item::ty::InductiveTypeSynDecl,
    crate::decl::major_item::ty::StructureTypeSynNodeDecl,
    crate::decl::major_item::ty::StructureTypeSynDecl,
    crate::decl::major_item::ty::ExternTypeSynNodeDecl,
    crate::decl::major_item::ty::ExternTypeSynDecl,
    crate::decl::major_item::ty::UnionTypeSynNodeDecl,
    crate::decl::major_item::ty::UnionTypeSynDecl,
    // - trait
    crate::decl::major_item::TraitSynNodeDecl,
    crate::decl::major_item::trai_syn_node_decl,
    crate::decl::major_item::TraitSynDecl,
    crate::decl::major_item::trai_syn_decl,
    // - fugitive
    crate::decl::major_item::fugitive_syn_node_decl,
    crate::decl::major_item::fugitive_syn_decl,
    crate::decl::major_item::MajorValSynNodeDecl,
    crate::decl::major_item::MajorValSynDecl,
    crate::decl::major_item::MajorFnSynNodeDecl,
    crate::decl::major_item::MajorFnSynDecl,
    crate::decl::major_item::MajorGnSynNodeDecl,
    crate::decl::major_item::MajorGnSynDecl,
    crate::decl::major_item::TypeAliasSynNodeDecl,
    crate::decl::major_item::TypeAliasSynDecl,
    // - impl block
    crate::decl::impl_block::TypeImplBlockSynNodeDecl,
    crate::decl::impl_block::ty_impl_block_syn_node_decl,
    crate::decl::impl_block::TypeImplBlockSynDecl,
    crate::decl::impl_block::ty_impl_block_syn_decl,
    crate::decl::impl_block::TraitForTypeImplBlockSynNodeDecl,
    crate::decl::impl_block::trai_for_ty_impl_block_syn_node_decl,
    crate::decl::impl_block::TraitForTypeImplBlockSynDecl,
    crate::decl::impl_block::trai_for_ty_impl_block_syn_decl,
    crate::decl::impl_block::IllFormedImplBlockSynNodeDecl,
    crate::decl::impl_block::ill_formed_impl_block_syn_node_decl,
    // - variant
    crate::decl::ty_variant_syn_node_decl,
    crate::decl::ty_variant_syn_decl,
    crate::decl::TypeUnitVariantSynNodeDecl,
    crate::decl::TypeUnitVariantSynDecl,
    crate::decl::TypePropsVariantSynNodeDecl,
    crate::decl::TypePropsVariantSynDecl,
    crate::decl::TypeTupleVariantSynNodeDecl,
    crate::decl::TypeTupleVariantSynDecl,
    // - associated items
    // -- type item
    crate::decl::ty_item_syn_node_decl,
    crate::decl::ty_item_syn_decl,
    crate::decl::TypeAssocFnSynNodeDecl,
    crate::decl::TypeAssocFnSynDecl,
    crate::decl::TypeMethodFnSynNodeDecl,
    crate::decl::TypeMethodFnSynDecl,
    crate::decl::TypeAssocTypeSynNodeDecl,
    crate::decl::TypeAssocTypeSynDecl,
    crate::decl::TypeAssocValSynNodeDecl,
    crate::decl::TypeAssocValSynDecl,
    crate::decl::TypeMemoizedFieldSynNodeDecl,
    crate::decl::TypeMemoizedFieldSynDecl,
    // -- trait item
    crate::decl::trai_item_syn_node_decl,
    crate::decl::trai_item_syn_decl,
    crate::decl::TraitAssocFnSynNodeDecl,
    crate::decl::TraitAssocFnSynDecl,
    crate::decl::TraitMethodFnSynNodeDecl,
    crate::decl::TraitMethodFnSynDecl,
    crate::decl::TraitAssocTypeSynNodeDecl,
    crate::decl::TraitAssocTypeSynDecl,
    crate::decl::TraitAssocValSynNodeDecl,
    crate::decl::TraitAssocValSynDecl,
    // -- trait for type item
    crate::decl::trai_for_ty_item_syn_decl,
    crate::decl::trai_for_ty_item_syn_node_decl,
    crate::decl::TraitForTypeAssocFnSynNodeDecl,
    crate::decl::TraitForTypeAssocFnSynDecl,
    crate::decl::TraitForTypeMethodFnSynNodeDecl,
    crate::decl::TraitForTypeMethodFnSynDecl,
    crate::decl::TraitForTypeAssocTypeSynNodeDecl,
    crate::decl::TraitForTypeAssocTypeSynDecl,
    crate::decl::TraitForTypeAssocValSynNodeDecl,
    crate::decl::TraitForTypeAssocValSynDecl,
    // -- ill formed item
    crate::decl::IllFormedItemSynNodeDecl,
    // attr
    crate::decl::attr::derive::DeriveAttrSynNodeDecl,
    crate::decl::attr::derive::DeriveAttrSynDecl,
    crate::decl::attr::backprop::BackpropAttrSynNodeDecl,
    crate::decl::attr::backprop::BackwardAttrSynDecl,
    crate::decl::attr::effect::EffectAttrSynNodeDecl,
    crate::decl::attr::effect::EffectAttrSynDecl,
    crate::decl::attr_syn_node_decl,
    crate::decl::attr_syn_decl,
    // sheet
    crate::sheet::SynNodeDeclSheet,
    crate::sheet::syn_node_decl_sheet,
    crate::sheet::SynDeclSheet,
    crate::sheet::syn_decl_sheet,
);
