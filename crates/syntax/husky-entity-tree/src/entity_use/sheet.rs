use crate::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleUseSheet {
    module_uses: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_use_sheet(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> VfsResult<ModuleUseSheet> {
    ModuleUseCollector::new(db, crate_path).collect_all()
}

#[test]
fn module_use_sheet_works() {
    DB::expect_test_crates("module_use_sheet", |db, crate_path| {
        module_use_sheet(db, crate_path)
    })
}

pub struct ModuleUseCollector<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    root: EntityPath,
    uses: HashMap<EntityPath, VfsResult<Vec<EntityTreeIdx>>>,
    modified_modules: HashSet<EntityPath>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

impl<'a> ModuleUseCollector<'a> {
    fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> Self {
        todo!();
        // let primal_module_use_sheet = primal_module_use_sheet(db, crate_path);
        Self {
            db,
            crate_path,
            root: db.it_entity_path(EntityPathData::CrateRoot(crate_path)),
            uses: todo!(),
            modified_modules: Default::default(),
            errors: vec![],
        }
    }

    fn collect_all(&mut self) -> VfsResult<ModuleUseSheet> {
        todo!()
    }
}
