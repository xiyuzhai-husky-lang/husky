use crate::*;

use husky_entity_path::ItemPath;
use husky_vfs::*;
use vec_like::VecSet;

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn submodules(
    db: &dyn EntitySynTreeDb,
    module_path: ModulePath,
) -> VfsResult<Vec<SubmodulePath>> {
    let ast_sheet = db.ast_sheet(module_path)?;
    Ok(ast_sheet
        .top_level_asts_iter()
        .filter_map(|ast| match ast {
            Ast::Identifiable { block, .. } => match block {
                DefnBlock::Submodule { path, .. } => Some(*path),
                _ => None,
            },
            _ => None,
        })
        .collect())
}

/// all modules, must be included in module tree
#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn all_modules_within_crate(
    db: &dyn EntitySynTreeDb,
    crate_path: CratePath,
) -> VecSet<ModulePath> {
    let root = ModulePath::new_root(db, crate_path);
    let mut all_modules = VecSet::default();
    all_modules.insert(root);
    collect_all_modules(db, root, &mut all_modules);
    all_modules
}

fn collect_all_modules(
    db: &dyn EntitySynTreeDb,
    root: ModulePath,
    all_modules: &mut VecSet<ModulePath>,
) {
    if let Ok(submodules) = submodules(db, root).as_ref() {
        for submodule in submodules {
            all_modules.insert(**submodule);
            collect_all_modules(db, **submodule, all_modules)
        }
    }
}

#[test]
fn submodules_works() {
    DB::default().ast_expect_test_debug_with_db(DB::submodules, &AstTestConfig::new("submodules"))
}

#[test]
fn all_modules_works() {
    DB::default().ast_expect_test_debug_with_db(
        EntitySynTreeDb::all_modules_within_crate,
        &AstTestConfig::new("all_modules_within_crate"),
    )
}
