use crate::*;
use vec_like::{VecMap, VecPairMap};

#[salsa::tracked(jar = EntitySymbolJar, return_ref)]
pub(crate) fn entity_tree_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<EntitySymbolBundle> {
    Ok(EntitySymbolCollector::new(db, crate_path)?.collect_all())
}

#[test]
fn entity_tree_bundle_works() {
    DB::expect_test_crates_debug_result_with_db(
        "entity_tree_bundle",
        |db, crate_path| -> EntityTreeResult<_> {
            Ok(entity_tree_bundle(db, crate_path).as_ref()?)
        },
    )
}

#[salsa::tracked(jar = EntitySymbolJar)]
pub struct EntitySymbolBundle {
    sheets: VecPairMap<ModulePath, EntitySymbolSheet>,
}

impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntitySymbolBundle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
