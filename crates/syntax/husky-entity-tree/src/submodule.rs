use crate::*;

use husky_entity_path::EntityPath;
use husky_vfs::*;

pub(crate) fn subentities() -> Vec<EntityTree> {
    todo!()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn submodules(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<Vec<ModulePath>> {
    let entity_tree_sheet = db.entity_tree_sheet(module_path).as_ref()?;
    Ok(entity_tree_sheet
        .top_level_entities()
        .filter_map(|(_, _, card, path)| match card {
            EntityCard::Module => match path.data(db) {
                EntityPathData::Module(_) => todo!(),
                EntityPathData::Associated { parent, ident } => todo!(),
            },
            _ => None,
        })
        .collect())
}

/// all modules, must be included in module tree
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn all_modules_within_crate(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> VfsResult<Vec<ModulePath>> {
    let mut all_modules = vec![];
    let root = ModulePath::new_root(db, crate_path);
    collect_all_modules(db, root, &mut all_modules);
    Ok(all_modules)
}

fn collect_all_modules(db: &dyn EntityTreeDb, root: ModulePath, all_modules: &mut Vec<ModulePath>) {
    if let Ok(submodules) = submodules(db, root).as_ref() {
        for submodule in submodules {
            all_modules.push(*submodule);
            collect_all_modules(db, *submodule, all_modules)
        }
    }
}

#[test]
fn submodules_works() {
    DB::expect_test_probable_modules_debug_with_db("submodules", DB::submodules)
}

#[test]
fn all_modules_works() {
    DB::expect_test_crates_debug_with_db(
        "all_modules_within_crate",
        EntityTreeDb::all_modules_within_crate,
    )
}
