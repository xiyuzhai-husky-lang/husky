use crate::*;

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
    crate::decl::major_item::ty::EnumSynNodeDecl,
    crate::decl::major_item::ty::EnumSynDecl,
    crate::decl::major_item::ty::UnitStructSynNodeDecl,
    crate::decl::major_item::ty::UnitStructSynDecl,
    crate::decl::major_item::ty::TupleStructSynNodeDecl,
    crate::decl::major_item::ty::TupleStructSynDecl,
    crate::decl::major_item::ty::PropsStructSynNodeDecl,
    crate::decl::major_item::ty::PropsStructSynDecl,
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
    // - form
    crate::decl::major_item::form_syn_node_decl,
    crate::decl::major_item::form_syn_decl,
    crate::decl::major_item::MajorValSynNodeDecl,
    crate::decl::major_item::MajorValSynDecl,
    crate::decl::major_item::form::compterm::MajorConstSynNodeDecl,
    crate::decl::major_item::form::compterm::MajorConstSynDecl,
    crate::decl::major_item::form::r#static::MajorStaticSynNodeDecl,
    crate::decl::major_item::form::r#static::MajorStaticSynDecl,
    crate::decl::major_item::MajorFunctionRitchieSynNodeDecl,
    crate::decl::major_item::MajorFunctionRitchieSynDecl,
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
    crate::decl::TypeAssocRitchieSynNodeDecl,
    crate::decl::TypeAssocRitchieSynDecl,
    crate::decl::TypeMethodRitchieSynNodeDecl,
    crate::decl::TypeMethodRitchieSynDecl,
    crate::decl::TypeAssocTypeSynNodeDecl,
    crate::decl::TypeAssocTypeSynDecl,
    crate::decl::TypeAssocValSynNodeDecl,
    crate::decl::TypeAssocValSynDecl,
    crate::decl::TypeMemoizedFieldSynNodeDecl,
    crate::decl::TypeMemoizedFieldSynDecl,
    // -- trait item
    crate::decl::assoc_item::trai_item::trai_item_syn_node_decl,
    crate::decl::assoc_item::trai_item::trai_item_syn_decl,
    crate::decl::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieSynDecl,
    crate::decl::assoc_item::trai_item::method_ritchie::TraitMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_item::method_ritchie::TraitMethodRitchieSynDecl,
    crate::decl::assoc_item::trai_item::memo_field::TraitMemoizedFieldSynNodeDecl,
    crate::decl::assoc_item::trai_item::memo_field::TraitMemoizedFieldSynDecl,
    crate::decl::assoc_item::trai_item::assoc_static::TraitAssocStaticSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_static::TraitAssocStaticSynDecl,
    crate::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynDecl,
    crate::decl::assoc_item::trai_item::assoc_val::TraitAssocValSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_val::TraitAssocValSynDecl,
    // -- trait for type item
    crate::decl::trai_for_ty_item_syn_decl,
    crate::decl::trai_for_ty_item_syn_node_decl,
    crate::decl::TraitForTypeAssocRitchieSynNodeDecl,
    crate::decl::TraitForTypeAssocRitchieSynDecl,
    crate::decl::TraitForTypeMethodRitchieSynNodeDecl,
    crate::decl::TraitForTypeMethodRitchieSynDecl,
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
    crate::decl::attr::backprop::BackpropAttrSynDecl,
    crate::decl::attr::affect::AffectAttrSynNodeDecl,
    crate::decl::attr::affect::AffectAttrSynDecl,
    crate::decl::attr::marker::MarkerAttrSynNodeDecl,
    crate::decl::attr::marker::MarkerAttrSynDecl,
    crate::decl::attr::test::TestAttrSynNodeDecl,
    crate::decl::attr::test::TestAttrSynDecl,
    crate::decl::attr_syn_node_decl,
    crate::decl::attr_syn_decl,
    // sheet
    crate::sheet::SynNodeDeclSheet,
    crate::sheet::syn_node_decl_sheet,
    crate::sheet::SynDeclSheet,
    crate::sheet::syn_decl_sheet,
);
