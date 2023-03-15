use crate::*;
use thiserror::Error;
use vec_like::VecMap;
mod error;

pub use self::error::*;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_crate_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeBundleResult<EntityTreeCrateBundle> {
    Ok(EntityTreeCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn entity_tree_crate_bundle_works() {
    DB::default().ast_expect_test_debug_with_db(
        "entity_tree_bundle",
        |db, crate_path| -> EntityTreeBundleResult<_> {
            Ok(entity_tree_crate_bundle(db, crate_path)
                .as_ref()
                .map_err(|e| e.clone())?)
        },
    )
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityTreeCrateBundle {
    sheets: VecMap<EntityTreeSheet>,
    principal_entity_path_expr_arena: MajorPathExprArena,
}

impl EntityTreeCrateBundle {
    pub(crate) fn new(
        sheets: VecMap<EntityTreeSheet>,
        principal_entity_path_expr_arena: MajorPathExprArena,
    ) -> Self {
        Self {
            sheets,
            principal_entity_path_expr_arena,
        }
    }

    pub fn sheets(&self) -> &[EntityTreeSheet] {
        &self.sheets
    }

    pub fn all_ty_impl_blocks<'a>(&'a self) -> impl Iterator<Item = TypeImplBlock> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_ty_impl_blocks())
            .flatten()
    }

    pub fn all_ty_as_trai_impl_blocks<'a>(
        &'a self,
    ) -> impl Iterator<Item = TypeAsTraitImplBlock> + 'a {
        self.sheets
            .iter()
            .map(|sheet| sheet.all_ty_as_trai_impl_blocks())
            .flatten()
    }

    pub(crate) fn get_sheet(&self, module_path: ModulePath) -> Option<&EntityTreeSheet> {
        self.sheets.get_entry(module_path)
    }
}
