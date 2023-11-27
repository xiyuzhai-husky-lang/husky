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
    ItemSynNodePathId,
    UseSymbol,
    // module items
    trai_item_paths,
    ty_attrs,
    ty_attr_paths,
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
    crate::helpers::tokra_region::decl::item_syn_node_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::defn::DefnTokraRegion,
    crate::helpers::tokra_region::defn::SynDefnTokraRegionSourceMap,
    crate::helpers::tokra_region::defn::item_syn_defn_tokra_region_with_source_map,
);
