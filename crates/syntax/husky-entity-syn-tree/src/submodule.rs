use crate::*;

use husky_vfs::{error::VfsResult, *};
use salsa::test_utils::TestDb;
use vec_like::VecSet;

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn submodules(db: &dyn EntitySynTreeDb, module_path: ModulePath) -> Vec<SubmodulePath> {
    let ast_sheet = db.ast_sheet(module_path);
    ast_sheet
        .top_level_asts_iter()
        .filter_map(|ast| match ast {
            Ast::Identifiable { block, .. } => match block {
                DefnBlock::Submodule { path, .. } => Some(*path),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// all modules, must be included in module tree
#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn all_modules_within_crate(
    db: &dyn EntitySynTreeDb,
    crate_path: CratePath,
) -> VecSet<ModulePath> {
    let root = crate_path.root_module_path(db);
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
    for submodule in submodules(db, root) {
        all_modules.insert(**submodule);
        collect_all_modules(db, **submodule, all_modules)
    }
}

#[test]
fn submodules_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| db.submodules(module_path),
        &AstTestConfig::new("submodules"),
    )
}

#[test]
fn all_modules_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, crate_path| EntitySynTreeDb::all_modules_within_crate(db, crate_path),
        &AstTestConfig::new("all_modules_within_crate"),
    )
}
