use crate::*;
use vec_like::{VecMap, VecPairMap};

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> EntityTreeResult<EntityTreeBundle> {
    Ok(EntityTreeCollector::new(db, crate_path)?.collect_all())
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

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeBundle {
    sheets: VecMap<EntityTreeSheet>,
}

impl EntityTreeBundle {
    pub fn new(sheets: VecMap<EntityTreeSheet>) -> Self {
        Self { sheets }
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
