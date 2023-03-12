use crate::*;
use thiserror::Error;
use vec_like::VecMap;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum EntityTreeBundleError {
    #[error("from toolchain error")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error")]
    Prelude(#[from] PreludeError),
}

impl From<&EntityTreeBundleError> for EntityTreeBundleError {
    fn from(value: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntityTreeBundleError {
    fn fmt(
        &self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        todo!()
    }
}

pub type EntityTreeCrateBundleResult<T> = Result<T, EntityTreeBundleError>;
pub type EntityTreeCrateBundleResultRef<'a, T> = Result<T, &'a EntityTreeBundleError>;

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
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityTreeCrateBundle {
    sheets: VecMap<EntityTreeSheet>,
    principal_entity_path_expr_arena: MajorPathExprArena,
    impl_blocks: ImplBlockBundle,
}

impl EntityTreeCrateBundle {
    pub(crate) fn new(
        sheets: VecMap<EntityTreeSheet>,
        principal_entity_path_expr_arena: MajorPathExprArena,
        impl_blocks: ImplBlockBundle,
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

    pub fn impl_blocks(&self) -> &ImplBlockBundle {
        &self.impl_blocks
    }

    pub(crate) fn get_sheet(&self, module_path: ModulePath) -> Option<&EntityTreeSheet> {
        self.sheets.get_entry(module_path)
    }
}
