use crate::*;
use vec_like::VecMap;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> VfsResult<EntityTreeBundle> {
    Ok(EntityTreeCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn entity_tree_bundle_works() {
    DB::expect_test_crates_debug_result_with_db(
        "entity_tree_bundle",
        |db, crate_path| -> VfsResult<_> { Ok(entity_tree_bundle(db, crate_path).as_ref()?) },
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeBundle {
    sheets: VecMap<ModulePath, EntityTreeSheet>,
}

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntityTreeBundle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntityTreeBundle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
