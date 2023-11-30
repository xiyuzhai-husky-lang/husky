use super::*;
use husky_entity_syn_tree::helpers::paths::{crate_module_paths, module_item_paths};
use husky_vfs::{CratePath, PackagePath};
use vec_like::VecSet;

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub(crate) fn package_amazon_linkages(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> VecSet<Linkage> {
    let mut linkages: VecSet<Linkage> = Default::default();
    for &crate_path in package_path.crate_paths(db) {
        for &module_path in crate_module_paths(db, crate_path) {
            for &item_path in module_item_paths(db, module_path) {
                if let Some(linkage) = Linkage::from_item_path(item_path, db) {
                    linkages.insert(linkage)
                }
                // todo: type variants etc
            }
        }
    }
    linkages
}

#[test]
fn package_amazon_linkages_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        package_amazon_linkages,
        &AstTestConfig::new("package_amazon_linkages"),
    )
}
