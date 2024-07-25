use crate::*;
use husky_entity_path::path::MajorEntityPath;
use husky_vfs::path::module_path::SubmodulePath;

#[deprecated(note = "todo: move methods to different traits")]
pub trait EntityTreeDb {
    fn submodules(&self, module_path: ModulePath) -> &[SubmodulePath];
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath];
    fn item_syn_tree_bundle(&self, crate_path: CratePath) -> &EntityTreeCrateBundle;
    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> &EntityTreePresheet;
    fn item_syn_tree_sheet(&self, module_path: ModulePath) -> &EntityTreeSheet;
    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntityTreeResult<ModuleSymbolContext<'a>>;
    fn subitem_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> EntityTreeResult<SubitemPath>;
}

impl EntityTreeDb for ::salsa::Db {
    fn submodules(&self, module_path: ModulePath) -> &[SubmodulePath] {
        submodules(self, module_path)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath] {
        all_modules_within_crate(self, crate_path)
    }

    fn item_syn_tree_bundle(&self, crate_path: CratePath) -> &EntityTreeCrateBundle {
        item_tree_crate_bundle(self, crate_path)
    }

    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> &EntityTreePresheet {
        item_tree_presheet(self, module_path)
    }

    fn item_syn_tree_sheet(&self, module_path: ModulePath) -> &EntityTreeSheet {
        item_tree_sheet(self, module_path)
    }

    fn module_symbol_context<'a>(
        &'a self,
        module_path: ModulePath,
    ) -> EntityTreeResult<ModuleSymbolContext<'a>> {
        module_symbol_context(self, module_path)
    }

    fn subitem_path(&self, parent: MajorEntityPath, ident: Ident) -> EntityTreeResult<SubitemPath> {
        subitem_path(self, parent, ident)
    }
}

#[salsa::jar]
pub struct EntityTreeJar(
    crate::node::ItemSynNodePathId,
    UseSymbol,
    ParentSuperSymbol,
    // module items
    crate::node::major_item::trai::trai_item_paths,
    crate::node::assoc_item::trai_item::trai_item_syn_nodes,
    // ty_impl_blocks,
    crate::node::assoc_item::ty_item::ty_item_syn_node_paths,
    crate::node::assoc_item::ty_item::ty_item_paths_map,
    crate::node::impl_block::ty_impl_block::ty_impl_block_item_paths,
    crate::node::assoc_item::ty_item::ty_impl_block_items,
    // ty for ty item
    crate::node::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_item_paths,
    // variants
    crate::node::ty_variant::ty_variant_syn_nodes,
    crate::node::ty_variant::ty_variant_paths,
    // impl blocks
    // - type impl block
    // - trai for ty impl block
    crate::node::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_items,
    // attr
    crate::node::attr::item_attr_paths,
    crate::node::attr::item_attr_syn_nodes,
    // other
    item_tree_presheet,
    item_tree_crate_bundle,
    submodules,
    all_modules_within_crate,
    // prelude
    crate_specific_symbol_context,
    none_core_crate_universal_prelude,
    // helpers
    crate::helpers::ty_side_trai_for_ty_impl_block_paths_map,
    crate::helpers::TraitOrderedSet,
    crate::helpers::non_core_crate_prelude_trait_item_records,
    crate::helpers::module_specific_trait_item_records,
    crate::helpers::paths::module_item_syn_node_paths,
    crate::helpers::paths::module_item_paths,
    crate::helpers::paths::module_test_paths,
    crate::helpers::paths::module_submodule_item_paths,
    crate::helpers::paths::crate_module_paths,
    crate::helpers::paths::crate_item_paths,
    crate::helpers::paths::crate_test_paths,
    crate::helpers::paths::to_test_path,
    crate::helpers::trai_side_derive_any_trai_for_ty_impl_block_paths_map,
    crate::helpers::trai_side_path_leading_trai_for_ty_impl_block_paths_map,
    crate::helpers::tokra_region::crate_decl::CrateDeclTokraRegion,
    crate::helpers::tokra_region::crate_decl::CrateDeclTokraRegionSourceMap,
    crate::helpers::tokra_region::crate_decl::crate_decl_tokra_region,
    crate::helpers::tokra_region::item_decl::ItemDeclTokraRegion,
    crate::helpers::tokra_region::item_decl::item_syn_node_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::item_defn::ItemDefnTokraRegion,
    crate::helpers::tokra_region::item_defn::ItemDefnTokraRegionSourceMap,
    crate::helpers::tokra_region::item_defn::item_defn_tokra_region_with_source_map,
    crate::utils::item_syn_node_path_stem_inner,
    crate::utils::item_syn_node_path_stem_index,
);
