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
    // - crate
    crate::decl::crate_::crate_syn_node_decl,
    crate::decl::crate_::crate_syn_decl,
    crate::decl::crate_::lib::LibCrateSynNodeDecl,
    crate::decl::crate_::lib::LibCrateSynDecl,
    crate::decl::crate_::main::MainCrateSynNodeDecl,
    crate::decl::crate_::main::MainCrateSynDecl,
    crate::decl::crate_::requirements::RequirementsCrateSynNodeDecl,
    crate::decl::crate_::requirements::RequirementsCrateSynDecl,
    crate::decl::crate_::task::TaskCrateSynNodeDecl,
    crate::decl::crate_::task::TaskCrateSynDecl,
    // - submodule
    crate::decl::submodule::SubmoduleSynNodeDecl,
    crate::decl::submodule::submodule_syn_node_decl,
    crate::decl::submodule::SubmoduleSynDecl,
    crate::decl::submodule::submodule_decl,
    // - type
    crate::decl::major_item::ty::ty_node_decl,
    crate::decl::major_item::ty::ty_decl,
    crate::decl::major_item::ty::r#enum::EnumSynNodeDecl,
    crate::decl::major_item::ty::r#enum::EnumSynDecl,
    crate::decl::major_item::ty::unit_struct::UnitStructSynNodeDecl,
    crate::decl::major_item::ty::unit_struct::UnitStructSynDecl,
    crate::decl::major_item::ty::tuple_struct::TupleStructSynNodeDecl,
    crate::decl::major_item::ty::tuple_struct::TupleStructSynDecl,
    crate::decl::major_item::ty::props_struct::PropsStructSynNodeDecl,
    crate::decl::major_item::ty::props_struct::PropsStructSynDecl,
    crate::decl::major_item::ty::inductive::InductiveSynNodeDecl,
    crate::decl::major_item::ty::inductive::InductiveSynDecl,
    crate::decl::major_item::ty::structure::StructureSynNodeDecl,
    crate::decl::major_item::ty::structure::StructureSynDecl,
    crate::decl::major_item::ty::r#extern::ExternSynNodeDecl,
    crate::decl::major_item::ty::r#extern::ExternSynDecl,
    crate::decl::major_item::ty::r#union::UnionSynNodeDecl,
    crate::decl::major_item::ty::r#union::UnionSynDecl,
    // - trait
    crate::decl::major_item::trai::TraitSynNodeDecl,
    crate::decl::major_item::trai::trai_syn_node_decl,
    crate::decl::major_item::trai::TraitSynDecl,
    crate::decl::major_item::trai::trai_syn_decl,
    // - major form
    crate::decl::major_item::form::form_syn_node_decl,
    crate::decl::major_item::form::form_syn_decl,
    crate::decl::major_item::form::val::MajorValSynNodeDecl,
    crate::decl::major_item::form::val::MajorValSynDecl,
    crate::decl::major_item::form::compterm::MajorComptermSynNodeDecl,
    crate::decl::major_item::form::compterm::MajorComptermSynDecl,
    crate::decl::major_item::form::static_mut::MajorStaticMutSynNodeDecl,
    crate::decl::major_item::form::static_mut::MajorStaticMutSynDecl,
    crate::decl::major_item::form::static_var::MajorStaticVarSynNodeDecl,
    crate::decl::major_item::form::static_var::MajorStaticVarSynDecl,
    crate::decl::major_item::form::function_ritchie::MajorFunctionRitchieSynNodeDecl,
    crate::decl::major_item::form::function_ritchie::MajorFunctionRitchieSynDecl,
    crate::decl::major_item::form::ty_alias::TypeAliasSynNodeDecl,
    crate::decl::major_item::form::ty_alias::TypeAliasSynDecl,
    crate::decl::major_item::form::ty_var::TypeVarSynNodeDecl,
    crate::decl::major_item::form::ty_var::TypeVarSynDecl,
    // - impl block
    crate::decl::impl_block::ty_impl_block::TypeImplBlockSynNodeDecl,
    crate::decl::impl_block::ty_impl_block::ty_impl_block_syn_node_decl,
    crate::decl::impl_block::ty_impl_block::TypeImplBlockSynDecl,
    crate::decl::impl_block::ty_impl_block::ty_impl_block_syn_decl,
    crate::decl::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockSynNodeDecl,
    crate::decl::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_syn_node_decl,
    crate::decl::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockSynDecl,
    crate::decl::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_syn_decl,
    crate::decl::impl_block::ill_formed::IllFormedImplBlockSynNodeDecl,
    crate::decl::impl_block::ill_formed::ill_formed_impl_block_syn_node_decl,
    // - variant
    crate::decl::ty_variant::ty_variant_syn_node_decl,
    crate::decl::ty_variant::ty_variant_syn_decl,
    crate::decl::ty_variant::unit_ty_variant::TypeUnitVariantSynNodeDecl,
    crate::decl::ty_variant::unit_ty_variant::TypeUnitVariantSynDecl,
    crate::decl::ty_variant::props_ty_variant::TypePropsVariantSynNodeDecl,
    crate::decl::ty_variant::props_ty_variant::TypePropsVariantSynDecl,
    crate::decl::ty_variant::tuple_ty_variant::TypeTupleVariantSynNodeDecl,
    crate::decl::ty_variant::tuple_ty_variant::TypeTupleVariantSynDecl,
    // - associated items
    // -- type item
    crate::decl::assoc_item::ty_item::ty_item_syn_node_decl,
    crate::decl::assoc_item::ty_item::ty_item_syn_decl,
    crate::decl::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieSynDecl,
    crate::decl::assoc_item::ty_item::method_ritchie::TypeMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::ty_item::method_ritchie::TypeMethodRitchieSynDecl,
    crate::decl::assoc_item::ty_item::assoc_ty::TypeAssocTypeSynNodeDecl,
    crate::decl::assoc_item::ty_item::assoc_ty::TypeAssocTypeSynDecl,
    crate::decl::assoc_item::ty_item::assoc_val::TypeAssocValSynNodeDecl,
    crate::decl::assoc_item::ty_item::assoc_val::TypeAssocValSynDecl,
    crate::decl::assoc_item::ty_item::memo::TypeMemoizedFieldSynNodeDecl,
    crate::decl::assoc_item::ty_item::memo::TypeMemoizedFieldSynDecl,
    // -- trait item
    crate::decl::assoc_item::trai_item::trai_item_syn_node_decl,
    crate::decl::assoc_item::trai_item::trai_item_syn_decl,
    crate::decl::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieSynDecl,
    crate::decl::assoc_item::trai_item::method_ritchie::TraitMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_item::method_ritchie::TraitMethodRitchieSynDecl,
    crate::decl::assoc_item::trai_item::memo::TraitMemoizedFieldSynNodeDecl,
    crate::decl::assoc_item::trai_item::memo::TraitMemoizedFieldSynDecl,
    crate::decl::assoc_item::trai_item::assoc_static_mut::TraitAssocStaticMutSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_static_mut::TraitAssocStaticMutSynDecl,
    crate::decl::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarSynDecl,
    crate::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynDecl,
    crate::decl::assoc_item::trai_item::assoc_val::TraitAssocValSynNodeDecl,
    crate::decl::assoc_item::trai_item::assoc_val::TraitAssocValSynDecl,
    // -- trait for type item
    crate::decl::assoc_item::trai_for_ty_item::trai_for_ty_item_syn_decl,
    crate::decl::assoc_item::trai_for_ty_item::trai_for_ty_item_syn_node_decl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_static_mut::TraitForTypeAssocStaticMutSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_static_mut::TraitForTypeAssocStaticMutSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_static_var::TraitForTypeAssocStaticVarSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_static_var::TraitForTypeAssocStaticVarSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValSynDecl,
    crate::decl::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieSynNodeDecl,
    crate::decl::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieSynDecl,
    // -- ill formed item
    crate::decl::assoc_item::ill_formed_item::IllFormedItemSynNodeDecl,
    // attr
    crate::decl::attr::affect::AffectAttrSynNodeDecl,
    crate::decl::attr::affect::AffectAttrSynDecl,
    crate::decl::attr::backprop::BackpropAttrSynNodeDecl,
    crate::decl::attr::backprop::BackpropAttrSynDecl,
    crate::decl::attr::dep::DepAttrSynNodeDecl,
    crate::decl::attr::dep::DepAttrSynDecl,
    crate::decl::attr::derive::DeriveAttrSynNodeDecl,
    crate::decl::attr::derive::DeriveAttrSynDecl,
    crate::decl::attr::proj::ProjAttrSynNodeDecl,
    crate::decl::attr::proj::ProjAttrSynDecl,
    crate::decl::attr::singleton::SingletonAttrSynNodeDecl,
    crate::decl::attr::singleton::SingletonAttrSynDecl,
    crate::decl::attr::mark::MarkAttrSynNodeDecl,
    crate::decl::attr::mark::MarkerAttrSynDecl,
    crate::decl::attr::task::TaskAttrSynNodeDecl,
    crate::decl::attr::task::TaskAttrSynDecl,
    crate::decl::attr::test::TestAttrSynNodeDecl,
    crate::decl::attr::test::TestAttrSynDecl,
    crate::decl::attr::attr_syn_node_decl,
    crate::decl::attr::attr_syn_decl,
    // sheet
    crate::sheet::SynNodeDeclSheet,
    crate::sheet::syn_node_decl_sheet,
    crate::sheet::SynDeclSheet,
    crate::sheet::syn_decl_sheet,
);
