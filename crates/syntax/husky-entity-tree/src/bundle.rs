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

pub type EntityTreeBundleResult<T> = Result<T, EntityTreeBundleError>;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeBundleResult<EntityTreeBundle> {
    Ok(EntityTreeCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn entity_tree_bundle_works() {
    DB::expect_test_crates_debug_result_with_db(
        "entity_tree_bundle",
        |db, crate_path| -> EntityTreeBundleResult<_> {
            Ok(entity_tree_bundle(db, crate_path)
                .as_ref()
                .map_err(|e| e.clone())?)
        },
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeBundle {
    sheets: VecMap<EntityTreeSheet>,
    principal_entity_path_expr_arena: PrincipalEntityPathExprArena,
    impl_block_arena: ImplBlockArena,
}

impl EntityTreeBundle {
    pub(crate) fn new(
        sheets: VecMap<EntityTreeSheet>,
        principal_entity_path_expr_arena: PrincipalEntityPathExprArena,
        impl_block_arena: ImplBlockArena,
    ) -> Self {
        Self {
            sheets,
            principal_entity_path_expr_arena,
            impl_block_arena,
        }
    }

    pub fn sheets(&self) -> &[EntityTreeSheet] {
        &self.sheets
    }

    pub(crate) fn get_sheet(&self, module_path: ModulePath) -> Option<&EntityTreeSheet> {
        self.sheets.get_entry(module_path)
    }
}

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntityTreeBundle {
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

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityTreeBundle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}
