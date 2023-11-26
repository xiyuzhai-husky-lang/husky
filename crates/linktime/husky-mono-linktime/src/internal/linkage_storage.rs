use super::*;
use husky_rust_transpilation::{db::RustTranspilationJar, transpile::TranspileToFsFull};
use husky_vfs::PackagePath;

pub struct MonoLinkageStorage {}

impl MonoLinkageStorage {
    pub(super) fn generate(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        let _ = target_path.transpile_to_fs_full(db);
        Self {}
    }
}

#[test]
fn generate_linkage_storage_works() {
    // use husky_print_utils::*;
    // use husky_standard_dev_comptime_db::StandardDevComptimeDb;
    // use salsa::DebugWithDb;
    // let mut db = StandardDevComptimeDb::default();
    // db.vfs_plain_test(
    //     |db, package_path: PackagePath| {
    //         MonoLinkageStorage::generate(LinktimeTargetPath::new_package(package_path, db), db);
    //     },
    //     &VfsTestConfig::new("generate_linkage_storage")
    //         .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary),
    // );
}
