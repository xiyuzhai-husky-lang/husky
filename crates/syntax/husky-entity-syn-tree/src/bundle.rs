use crate::*;
use thiserror::Error;
use vec_like::VecMap;
mod error;

pub use self::error::*;

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn item_tree_crate_bundle(
    db: &::salsa::Db,
    crate_path: CratePath,
) -> EntitySynTreeCrateBundle {
    EntityTreeCollector::new(db, crate_path).collect_all()
}

#[test]
fn item_tree_crate_bundle_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, crate_path| item_tree_crate_bundle(db, crate_path),
        &AstTestConfig::new("item_tree_bundle"),
    )
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
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

    pub fn all_impl_block_ill_forms<'a>(
        &'a self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = &'a ImplBlockIllForm> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_ill_formed_impl_block_syn_nodes(db))
            .flatten()
    }

    pub(crate) fn trai_for_ty_impl_block_paths_filtered_by_trai_path<'a>(
        &'a self,
        db: &'a ::salsa::Db,
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
        db: &'a ::salsa::Db,
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
