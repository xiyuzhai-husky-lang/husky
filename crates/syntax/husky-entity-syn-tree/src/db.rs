use crate::*;
use husky_vfs::{error::VfsResult, *};

pub trait EntitySynTreeDb {
    fn submodules(&self, module_path: ModulePath) -> &[SubmodulePath];
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath];
    fn item_syn_tree_bundle(&self, crate_path: CratePath) -> &EntitySynTreeCrateBundle;
    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> &EntitySynTreePresheet;
    fn item_syn_tree_sheet(&self, module_path: ModulePath) -> &EntitySynTreeSheet;
    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<ModuleSymbolContext<'a>>;
    fn subitem_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> EntitySynTreeResult<SubitemPath>;
}

impl EntitySynTreeDb for ::salsa::Db {
    fn submodules(&self, module_path: ModulePath) -> &[SubmodulePath] {
        submodules(self, module_path)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath] {
        all_modules_within_crate(self, crate_path)
    }

    fn item_syn_tree_bundle(&self, crate_path: CratePath) -> &EntitySynTreeCrateBundle {
        item_tree_crate_bundle(self, crate_path)
    }

    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> &EntitySynTreePresheet {
        item_tree_presheet(self, module_path)
    }

    fn item_syn_tree_sheet(&self, module_path: ModulePath) -> &EntitySynTreeSheet {
        item_tree_sheet(self, module_path)
    }

    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<ModuleSymbolContext<'a>> {
        module_symbol_context(self, module_path)
    }

    fn subitem_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> EntitySynTreeResult<SubitemPath> {
        subitem_path(self, parent, identifier)
    }
}

#[salsa::jar(db = EntitySynTreeDb)]
pub struct EntitySynTreeJar(
    IllFormedImplBlockSynNodePath,
    SubmoduleSynNodePath,
    SubmoduleSynNode,
    submodule_syn_node,
    MajorItemSynNode,
    UseSymbol,
    // module items
    TraitSynNodePath,
    trai_node,
    trai_item_paths,
    TypeSynNodePath,
    ty_node,
    ty_attrs,
    ty_attr_paths,
    FugitiveSynNodePath,
    fugitive_syn_node,
    // ty variant
    TypeVariantSynNodePath,
    // associated items
    TypeItemSynNodePath,
    TypeItemSynNode,
    ty_item_syn_node,
    trai_item_syn_nodes,
    trai_item_syn_node,
    TraitItemSynNodePath,
    TraitItemSynNode,
    TraitForTypeItemSynNodePath,
    TraitForTypeItemSynNode,
    trai_for_ty_item_syn_node,
    IllFormedItemSynNodePath,
    IllFormedItemSynNode,
    TypeVariantSynNode,
    ty_variant_syn_node,
    // ty_impl_blocks,
    ty_item_syn_node_paths,
    ty_item_paths_map,
    ty_impl_block_item_paths,
    trai_for_ty_impl_block_items,
    trai_for_ty_impl_block_item_paths,
    // variants
    ty_variant_syn_nodes,
    ty_variant_paths,
    // impl blocks
    // - type impl block
    TypeImplBlockSynNode,
    ty_impl_block_syn_node,
    ty_impl_block_items,
    // - trait for type impl block
    TraitForTypeImplBlockSynNode,
    trai_for_ty_impl_block_syn_node,
    // - ill formed impl block
    IllFormedImplBlockSynNode,
    ill_formed_impl_block_syn_node,
    // attr
    attr_node,
    AttrSynNodePath,
    AttrSynNode,
    // other
    item_tree_presheet,
    item_tree_crate_bundle,
    submodules,
    module_subitem_path,
    all_modules_within_crate,
    // prelude
    crate_specific_prelude,
    none_core_crate_universal_prelude,
    // helpers
    crate::helpers::ty_side_trai_for_ty_impl_block_paths_map,
    crate::helpers::trai_item_table,
    crate::helpers::TraitOrderedSet,
    crate::helpers::non_core_crate_prelude_trait_items_table,
    crate::helpers::module_specific_trait_items_table,
    crate::helpers::paths::module_item_paths,
    crate::helpers::paths::module_item_syn_node_paths,
    crate::helpers::paths::crate_module_paths,
    crate::helpers::paths::module_submodule_paths,
    crate::helpers::trai_side_derive_any_trai_for_ty_impl_block_paths_map,
    crate::helpers::trai_side_path_leading_trai_for_ty_impl_block_paths_map,
    crate::helpers::tokra_region::SnippetTokraRegion,
    crate::helpers::tokra_region::DeclTokraRegion,
    crate::helpers::tokra_region::submodule_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::submodule_decl_tokra_region,
    crate::helpers::tokra_region::trai_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::trai_decl_tokra_region,
    crate::helpers::tokra_region::ty_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::ty_decl_tokra_region,
    crate::helpers::tokra_region::fugitive_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::fugitive_decl_tokra_region,
    crate::helpers::tokra_region::ty_variant_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::ty_variant_decl_tokra_region,
    crate::helpers::tokra_region::ty_impl_block_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::ty_impl_block_decl_tokra_region,
    crate::helpers::tokra_region::trai_for_ty_impl_block_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::trai_for_ty_impl_block_decl_tokra_region,
    crate::helpers::tokra_region::ill_formed_impl_block_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::ill_formed_impl_block_decl_tokra_region,
    crate::helpers::tokra_region::ty_item_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::ty_item_decl_tokra_region,
    crate::helpers::tokra_region::trai_item_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::trai_item_decl_tokra_region,
    crate::helpers::tokra_region::trai_for_ty_item_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::trai_for_ty_item_decl_tokra_region,
    crate::helpers::tokra_region::ill_formed_item_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::ill_formed_item_decl_tokra_region,
    crate::helpers::tokra_region::attr_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::attr_decl_tokra_region,
    // defn
    crate::helpers::tokra_region::DefnTokraRegion,
    crate::helpers::tokra_region::SynDefnTokraRegionSourceMap,
    crate::helpers::tokra_region::fugitive_defn_tokra_region_with_source_map,
    crate::helpers::tokra_region::fugitive_defn_tokra_region,
    crate::helpers::tokra_region::ty_item_defn_tokra_region_with_source_map,
    crate::helpers::tokra_region::ty_item_defn_tokra_region,
    crate::helpers::tokra_region::trai_item_defn_tokra_region_with_source_map,
    crate::helpers::tokra_region::trai_item_defn_tokra_region,
    crate::helpers::tokra_region::trai_for_ty_item_defn_tokra_region_with_source_map,
    crate::helpers::tokra_region::trai_for_ty_item_defn_tokra_region,
    crate::helpers::tokra_region::ill_formed_item_defn_tokra_region_with_source_map,
    crate::helpers::tokra_region::ill_formed_item_defn_tokra_region,
);
