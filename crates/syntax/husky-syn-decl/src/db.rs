use crate::*;
use husky_coword::Ident;
use husky_entity_syn_tree::EntitySynTreeResult;
use husky_print_utils::p;
use husky_vfs::ModulePath;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait SynDeclDb: DbWithJar<SynDeclJar> + SynExprDb {
    fn syn_node_decl_sheet(&self, module_path: ModulePath)
        -> EntitySynTreeResult<SynNodeDeclSheet>;

    fn syn_decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<SynDeclSheet>;
}

impl<Db> SynDeclDb for Db
where
    Db: DbWithJar<SynDeclJar> + SynExprDb,
{
    fn syn_node_decl_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<SynNodeDeclSheet> {
        syn_node_decl_sheet(self, module_path)
    }

    fn syn_decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<SynDeclSheet> {
        syn_decl_sheet(self, module_path)
    }
}

#[salsa::jar(db = SynDeclDb)]
pub struct SynDeclJar(
    // decl
    // - submodule
    SubmoduleSynNodeDecl,
    submodule_syn_node_decl,
    SubmoduleSynDecl,
    submodule_decl,
    // - type
    ty_node_decl,
    ty_decl,
    EnumTypeSynNodeDecl,
    EnumTypeSynDecl,
    UnitStructTypeSynNodeDecl,
    UnitStructTypeSynDecl,
    TupleStructTypeSynNodeDecl,
    TupleStructTypeSynDecl,
    PropsStructTypeSynNodeDecl,
    PropsStructTypeSynDecl,
    RecordTypeSynNodeDecl,
    RecordTypeSynDecl,
    InductiveTypeSynNodeDecl,
    InductiveTypeSynDecl,
    StructureTypeSynNodeDecl,
    StructureTypeSynDecl,
    ExternTypeSynNodeDecl,
    ExternTypeSynDecl,
    UnionTypeSynNodeDecl,
    UnionTypeSynDecl,
    // - trait
    TraitSynNodeDecl,
    trai_syn_node_decl,
    TraitSynDecl,
    trai_syn_decl,
    // - form
    fugitive_syn_node_decl,
    fugitive_syn_decl,
    ValSynNodeDecl,
    ValSynDecl,
    FnSynNodeDecl,
    FunctionFnSynDecl,
    GnSynNodeDecl,
    GnSynDecl,
    TypeAliasSynNodeDecl,
    TypeAliasSynDecl,
    // - impl block
    TypeImplBlockSynNodeDecl,
    ty_impl_block_syn_node_decl,
    TypeImplBlockSynDecl,
    ty_impl_block_syn_decl,
    TraitForTypeImplBlockSynNodeDecl,
    trai_for_ty_impl_block_syn_node_decl,
    TraitForTypeImplBlockSynDecl,
    trai_for_ty_impl_block_syn_decl,
    IllFormedImplBlockSynNodeDecl,
    ill_formed_impl_block_syn_node_decl,
    // - variant
    ty_variant_syn_node_decl,
    ty_variant_syn_decl,
    UnitTypeVariantSynNodeDecl,
    UnitTypeVariantSynDecl,
    PropsTypeVariantSynNodeDecl,
    PropsTypeVariantSynDecl,
    TupleTypeVariantSynNodeDecl,
    TupleTypeVariantSynDecl,
    // - associated items
    // -- type item
    ty_item_syn_node_decl,
    ty_item_syn_decl,
    TypeAssociatedFnSynNodeDecl,
    TypeAssociatedFnSynDecl,
    TypeMethodFnSynNodeDecl,
    TypeMethodFnSynDecl,
    TypeAssociatedTypeSynNodeDecl,
    TypeAssociatedTypeSynDecl,
    TypeAssociatedValSynNodeDecl,
    TypeAssociatedValSynDecl,
    TypeMemoizedFieldSynNodeDecl,
    TypeMemoizedFieldSynDecl,
    // -- trait item
    trai_item_syn_node_decl,
    TraitAssociatedFnSynNodeDecl,
    TraitAssociatedFnSynDecl,
    TraitMethodFnSynNodeDecl,
    TraitMethodFnSynDecl,
    TraitAssociatedTypeSynNodeDecl,
    TraitAssociatedTypeSynDecl,
    TraitAssociatedValSynNodeDecl,
    TraitAssociatedValSynDecl,
    // -- trait for type item
    trai_for_ty_item_syn_node_decl,
    trai_for_ty_item_syn_decl,
    TraitForTypeAssociatedFnSynNodeDecl,
    TraitForTypeAssociatedFnSynDecl,
    TraitForTypeMethodFnSynNodeDecl,
    TraitForTypeMethodFnSynDecl,
    TraitForTypeAssociatedTypeSynNodeDecl,
    TraitForTypeAssociatedTypeSynDecl,
    TraitForTypeAssociatedValSynNodeDecl,
    TraitForTypeAssociatedValSynDecl,
    // -- ill formed item
    IllFormedItemSynNodeDecl,
    // attr
    DeriveAttrSynDecl,
    DeriveAttrSynNodeDecl,
    attr_syn_node_decl,
    attr_syn_decl,
    // sheet
    SynNodeDeclSheet,
    syn_node_decl_sheet,
    SynDeclSheet,
    syn_decl_sheet,
);
