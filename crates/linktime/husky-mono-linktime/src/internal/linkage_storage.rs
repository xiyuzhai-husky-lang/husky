use super::*;
use husky_rust_transpilation::db::RustTranspilationDb;

pub struct MonoLinkageStorage {}

impl MonoLinkageStorage {
    pub(super) fn generate<Db: ?Sized>(target_crate: CratePath, db: &Db) -> Self
    where
        Db: RustTranspilationDb,
    {
        todo!();
        Self {}
    }
}
#[test]
fn generate_linkage_storage_works() {
    use husky_standard_dev_comptime_db::StandardDevComptimeDb;
    let mut db = StandardDevComptimeDb::default();
    db.vfs_plain_test(
        |db, crate_path| {
            MonoLinkageStorage::generate(crate_path, db);
        },
        &VfsTestConfig::new("generate_linkage_storage")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary),
    );
}
