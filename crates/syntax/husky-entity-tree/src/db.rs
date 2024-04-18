use crate::*;
use husky_vfs::*;

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

    fn subitem_path(
        &self,
        parent: MajorEntityPath,
        identifier: Ident,
    ) -> EntityTreeResult<SubitemPath> {
        subitem_path(self, parent, identifier)
    }
}

#[salsa::jar]
pub struct EntityTreeJar(
    ItemSynNodePathId,
    UseSymbol,
    ParentSuperSymbol,
    // module items
    trai_item_paths,
    trai_item_syn_nodes,
    // ty_impl_blocks,
    ty_item_syn_node_paths,
    ty_item_paths_map,
    ty_impl_block_item_paths,
    ty_impl_block_items,
    // ty for ty item
    trai_for_ty_impl_block_item_paths,
    // variants
    ty_variant_syn_nodes,
    ty_variant_paths,
    // impl blocks
    // - type impl block
    // - trai for ty impl block
    trai_for_ty_impl_block_items,
    // attr
    item_attr_paths,
    item_attr_syn_nodes,
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
    crate::helpers::paths::module_item_paths,
    crate::helpers::paths::module_item_syn_node_paths,
    crate::helpers::paths::crate_module_paths,
    crate::helpers::paths::module_submodule_item_paths,
    crate::helpers::paths::crate_item_paths,
    crate::helpers::trai_side_derive_any_trai_for_ty_impl_block_paths_map,
    crate::helpers::trai_side_path_leading_trai_for_ty_impl_block_paths_map,
    crate::helpers::tokra_region::SnippetTokraRegion,
    crate::helpers::tokra_region::DeclTokraRegion,
    crate::helpers::tokra_region::decl::item_syn_node_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::defn::DefnTokraRegion,
    crate::helpers::tokra_region::defn::DefnTokraRegionSourceMap,
    crate::helpers::tokra_region::defn::item_syn_defn_tokra_region_with_source_map,
    crate::helpers::ingredient::crate_ingredient_paths,
    crate::helpers::ingredient::item_path_ingredient_index,
);
