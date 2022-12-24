use crate::*;

use husky_entity_path::EntityPath;
use husky_vfs::*;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn submodules(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> VfsResult<Vec<ModulePath>> {
    let ast_sheet = db.ast_sheet(module_path)?;
    Ok(ast_sheet
        .top_level_asts_iter()
        .filter_map(|ast| match ast {
            Ast::Defn { entity_path, .. } => match (*entity_path)? {
                EntityPath::Module(module_path) => Some(module_path),
                EntityPath::ModuleItem(_)
                | EntityPath::AssociatedItem(_)
                | EntityPath::EnumVariant(_) => None,
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
    let root = ModulePath::new_root(db, crate_path);
    let mut all_modules = vec![root];
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
    DB::expect_test_probable_modules_debug_result_with_db("submodules", DB::submodules)
}

#[test]
fn all_modules_works() {
    DB::expect_test_crates_debug_result_with_db(
        "all_modules_within_crate",
        EntityTreeDb::all_modules_within_crate,
    )
}
