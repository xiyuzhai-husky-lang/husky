use super::*;
use husky_rust_transpilation::{
    db::{RustTranspilationDb, RustTranspilationJar},
    transpile::TranspileToFsFull,
};
use husky_vfs::PackagePath;

pub struct MonoLinkageStorage {}

impl MonoLinkageStorage {
    pub(super) fn generate<Db: ?Sized>(target_path: LinktimeTargetPath, db: &Db) -> Self
    where
        Db: RustTranspilationDb,
    {
        let db = <Db as salsa::DbWithJar<RustTranspilationJar>>::as_jar_db(db);
        let _ = target_path.transpile_to_fs_full(db);
        Self {}
    }
}
#[test]
fn generate_linkage_storage_works() {
    use husky_standard_dev_comptime_db::StandardDevComptimeDb;
    let mut db = StandardDevComptimeDb::default();
    db.vfs_plain_test(
        |db, package_path| {
            MonoLinkageStorage::generate(LinktimeTargetPath::Package(package_path), db);
        },
        &VfsTestConfig::new("generate_linkage_storage")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary),
    );
}
