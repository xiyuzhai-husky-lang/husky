use super::*;
use crate::jar::JavelinJar;
use husky_entity_syn_tree::helpers::paths::{crate_module_paths, module_item_paths};
use husky_vfs::{CratePath, PackagePath};
use vec_like::VecSet;

#[salsa::tracked(jar = JavelinJar, return_ref)]
pub(crate) fn package_amazon_javelins(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> VecSet<Javelin> {
    let mut javelins: VecSet<Javelin> = Default::default();
    for &crate_path in package_path.crate_paths(db) {
        for &module_path in crate_module_paths(db, crate_path) {
            for &item_path in module_item_paths(db, module_path) {
                if let Some(javelin) = Javelin::from_item_path(item_path, db) {
                    javelins.insert(javelin)
                }
                // todo: type variants etc
            }
        }
    }
    javelins
}

#[test]
fn package_amazon_javelins_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        package_amazon_javelins,
        &AstTestConfig::new("package_amazon_javelins"),
    )
}
