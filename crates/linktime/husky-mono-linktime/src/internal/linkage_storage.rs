use super::*;
use husky_cargo_utils::compile_workspace;
use husky_rust_transpilation::{db::RustTranspilationJar, transpile::TranspileToFsFull};
use husky_vfs::PackagePath;

pub struct MonoLinkageStorage {}

impl MonoLinkageStorage {
    pub(super) fn generate(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        match target_path.transpile_to_fs_full(db) {
            Ok(()) => (),
            Err(_) => todo!(),
        };
        compile_workspace(target_path.rust_workspace_manifest_path(db));
        Self {}
    }
}

#[test]
fn generate_linkage_storage_works() {
    use husky_dev_comptime::db::DevComptimeDb;
    use husky_print_utils::*;
    use salsa::DebugWithDb;
    let mut db = DevComptimeDb::default();
    db.vfs_plain_test(
        |db, package_path: PackagePath| {
            MonoLinkageStorage::generate(LinktimeTargetPath::new_package(package_path, db), db);
        },
        &VfsTestConfig::new("generate_linkage_storage")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary),
    );
}
