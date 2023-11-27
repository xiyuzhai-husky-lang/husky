use super::*;
use husky_entity_syn_tree::helpers::paths::{crate_module_paths, module_item_paths};
use husky_vfs::CratePath;
use vec_like::VecSet;

#[salsa::tracked(jar = LinkageJar, return_ref)]
fn crate_specific_linkages(db: &::salsa::Db, crate_path: CratePath) -> VecSet<Linkage> {
    let mut linkages: VecSet<Linkage> = Default::default();
    for &module_path in crate_module_paths(db, crate_path) {
        for &item_path in module_item_paths(db, module_path) {
            if let Some(linkage) = Linkage::from_item_path(item_path, db) {
                linkages.insert(linkage)
            }
            // todo: type variants etc
        }
    }
    linkages
}

#[test]
fn crate_specific_linkages_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        crate_specific_linkages,
        &AstTestConfig::new("crate_specific_linkages"),
    )
}
