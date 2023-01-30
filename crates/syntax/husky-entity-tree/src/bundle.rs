use crate::*;
use thiserror::Error;
use vec_like::{VecMap, VecPairMap};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeBundleError {
    #[error("from toolchain error")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error")]
    Prelude(#[from] PreludeError),
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntityTreeBundleError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

pub type EntityTreeCrateBundleResult<T> = Result<T, EntityTreeBundleError>;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_crate_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeCrateBundleResult<EntityTreeCrateBundle> {
    Ok(EntityTreeCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn entity_tree_crate_bundle_works() {
    DB::default().vfs_expect_test_debug_with_db(
        "entity_tree_bundle",
        |db, crate_path| -> EntityTreeCrateBundleResult<_> {
            Ok(entity_tree_crate_bundle(db, crate_path)
                .as_ref()
                .map_err(|e| e.clone())?)
        },
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeCrateBundle {
    sheets: VecMap<EntityTreeSheet>,
    principal_entity_path_expr_arena: MajorPathExprArena,
    impl_blocks: Vec<ImplBlock>,
}

impl EntityTreeCrateBundle {
    pub(crate) fn new(
        sheets: VecMap<EntityTreeSheet>,
        principal_entity_path_expr_arena: MajorPathExprArena,
        impl_blocks: Vec<ImplBlock>,
    ) -> Self {
        Self {
            sheets,
            principal_entity_path_expr_arena,
            impl_blocks,
        }
    }

    pub fn sheets(&self) -> &[EntityTreeSheet] {
        &self.sheets
    }

    pub(crate) fn get_sheet(&self, module_path: ModulePath) -> Option<&EntityTreeSheet> {
        self.sheets.get_entry(module_path)
    }

    pub fn impl_block_iter<'a>(&'a self) -> impl Iterator<Item = ImplBlock> + 'a {
        self.impl_blocks.iter().copied()
    }
}

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntityTreeCrateBundle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("EntityTreeBundle")
            .field("sheets", &self.sheets.debug_with(db, include_all_fields))
            .finish()
    }
}

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityTreeCrateBundle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}
