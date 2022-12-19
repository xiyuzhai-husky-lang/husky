use crate::*;
use husky_ast::AstIdx;
use husky_entity_path::EntityPath;
use husky_vfs::*;

pub(crate) fn subentities() -> Vec<EntityTree> {
    todo!()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn submodules(db: &dyn EntityTreeDb, module: EntityPath) -> VfsResult<Vec<EntityPath>> {
    let entity_tree_page0 = db.entity_tree_sheet(module).as_ref()?;
    Ok(entity_tree_page0
        .top_level_entities()
        .filter_map(|(_, card, path)| match card {
            EntityCard::Module => Some(path),
            _ => None,
        })
        .collect())
}

/// all modules, must be included in module tree
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn all_modules_within_crate(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> VfsResult<Vec<EntityPath>> {
    let mut all_modules = vec![];
    let root = db.it_entity_path(EntityPathData::CrateRoot(crate_path));
    collect_all_modules(db, root, &mut all_modules);
    Ok(all_modules)
}

fn collect_all_modules(db: &dyn EntityTreeDb, root: EntityPath, all_modules: &mut Vec<EntityPath>) {
    if let Ok(submodules) = submodules(db, root).as_ref() {
        for submodule in submodules {
            all_modules.push(*submodule);
            collect_all_modules(db, *submodule, all_modules)
        }
    }
}

#[test]
fn submodules_works() {
    DB::expect_test_probable_modules("submodules", DB::submodules)
}

#[test]
fn all_modules_works() {
    DB::expect_test_crates("all_modules_within_crate", DB::all_modules_within_crate)
}
