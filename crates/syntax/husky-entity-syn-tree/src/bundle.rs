use crate::*;
use thiserror::Error;
use vec_like::VecMap;
mod error;

pub use self::error::*;

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn item_tree_crate_bundle(
    db: &dyn EntitySynTreeDb,
    crate_path: CratePath,
) -> EntitySynTreeBundleResult<EntitySynTreeCrateBundle> {
    Ok(EntityTreeCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn item_tree_crate_bundle_works() {
    DB::default().ast_expect_test_debug_with_db(
        "item_tree_bundle",
        |db, crate_path| -> EntitySynTreeBundleResult<_> {
            Ok(item_tree_crate_bundle(db, crate_path)
                .as_ref()
                .map_err(|e| e.clone())?)
        },
    )
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct EntitySynTreeCrateBundle {
    sheets: VecMap<EntitySynTreeSheet>,
    principal_item_path_expr_arena: MajorPathExprArena,
}

impl EntitySynTreeCrateBundle {
    pub(crate) fn new(
        sheets: VecMap<EntitySynTreeSheet>,
        principal_item_path_expr_arena: MajorPathExprArena,
    ) -> Self {
        Self {
            sheets,
            principal_item_path_expr_arena,
        }
    }

    pub fn sheets(&self) -> &[EntitySynTreeSheet] {
        &self.sheets
    }

    pub fn all_ty_impl_block_syn_node_paths<'a>(
        &'a self,
    ) -> impl Iterator<Item = TypeImplBlockSynNodePath> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_ty_impl_block_syn_node_paths())
            .flatten()
    }

    pub fn all_ty_impl_block_syn_nodes<'a>(
        &'a self,
    ) -> impl Iterator<Item = TypeImplBlockSynNode> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_ty_impl_block_syn_nodes())
            .flatten()
    }

    pub fn all_ill_formed_impl_blocks<'a>(
        &'a self,
    ) -> impl Iterator<Item = IllFormedImplBlockSynNode> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_ill_formed_impl_block_syn_nodes())
            .flatten()
    }

    pub(crate) fn trai_for_ty_impl_block_paths_filtered_by_trai_path<'a>(
        &'a self,
        db: &'a dyn EntitySynTreeDb,
        trai_path: TraitPath,
    ) -> impl Iterator<Item = TraitForTypeImplBlockPath> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_trai_for_ty_impl_block_paths(db))
            .flatten()
            .filter(move |path| path.trai_path(db) == trai_path)
    }

    pub(crate) fn trai_for_ty_impl_block_paths_filtered_by_ty_path<'a>(
        &'a self,
        db: &'a dyn EntitySynTreeDb,
        ty_path: TypePath,
    ) -> impl Iterator<Item = TraitForTypeImplBlockPath> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_trai_for_ty_impl_block_paths(db))
            .flatten()
            .filter(move |path| path.ty_sketch(db) == TypeSketch::Path(ty_path))
    }

    pub(crate) fn get_sheet(&self, module_path: ModulePath) -> Option<&EntitySynTreeSheet> {
        self.sheets.get_entry(module_path)
    }
}
