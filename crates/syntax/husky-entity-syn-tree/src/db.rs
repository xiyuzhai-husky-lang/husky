use crate::*;

use husky_ast::AstDb;

use husky_entity_path::ItemPath;
use husky_entity_taxonomy::EntityKind;
use husky_manifest::ManifestDb;
use husky_vfs::*;

use salsa::DbWithJar;

pub trait EntitySynTreeDb: DbWithJar<EntitySynTreeJar> + AstDb + EntityPathDb + ManifestDb {
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[SubmodulePath]>;
    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath];
    fn item_syn_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntitySynTreeBundleResult<&EntitySynTreeCrateBundle>;
    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> VfsResult<&EntitySynTreePresheet>;
    fn item_syn_tree_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<&EntitySynTreeSheet>;
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

impl<T> EntitySynTreeDb for T
where
    T: DbWithJar<EntitySynTreeJar> + AstDb + EntityPathDb + ManifestDb,
{
    fn submodules(&self, module_path: ModulePath) -> VfsResult<&[SubmodulePath]> {
        Ok(submodules(self, module_path).as_ref()?)
    }

    fn all_modules_within_crate(&self, crate_path: CratePath) -> &[ModulePath] {
        all_modules_within_crate(self, crate_path)
    }

    fn item_syn_tree_bundle(
        &self,
        crate_path: CratePath,
    ) -> EntitySynTreeBundleResult<&EntitySynTreeCrateBundle> {
        Ok(item_tree_crate_bundle(self, crate_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn item_syn_tree_presheet(&self, module_path: ModulePath) -> VfsResult<&EntitySynTreePresheet> {
        Ok(item_tree_presheet(self, module_path).as_ref()?)
    }

    fn item_syn_tree_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<&EntitySynTreeSheet> {
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
    ty_decrs,
    ty_decr_paths,
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
    // decr
    decr_node,
    DecrSynNodePath,
    DecrSynNode,
    // other
    item_tree_presheet,
    item_tree_crate_bundle,
    crate::helpers::paths::module_item_paths,
    crate::helpers::paths::module_item_syn_node_paths,
    submodules,
    module_subitem_path,
    all_modules_within_crate,
    // prelude
    crate_specific_prelude,
    none_core_crate_universal_prelude,
    // helpers
    ty_side_trai_for_ty_impl_block_paths_map,
    trai_item_table,
    TraitOrderedSet,
    crate::helpers::non_core_crate_prelude_trait_items_table,
    crate::helpers::module_specific_trait_items_table,
    crate::helpers::trai_side_derive_any_trai_for_ty_impl_block_paths_map,
    crate::helpers::trai_side_path_leading_trai_for_ty_impl_block_paths_map,
    crate::helpers::tokra_region::SnippetTokraRegion,
    crate::helpers::tokra_region::DeclTokraRegion,
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
    crate::helpers::tokra_region::decr_decl_tokra_region_with_source_map,
    crate::helpers::tokra_region::decr_decl_tokra_region,
    // defn
    crate::helpers::tokra_region::SynDefnTokraRegion,
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
