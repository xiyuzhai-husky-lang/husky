use super::*;

pub(super) fn vfs_expect_test<Db, U>(
    db: &mut Db,
    f: impl Fn(&Db, U) -> String,
    config: &VfsTestConfig,
) where
    Db: VfsTestUtils + ?Sized,
    U: VfsTestUnit + salsa::DebugWithDb<Db>,
{
    let toolchain = db.dev_toolchain().unwrap();
    for test_domain in config.test_domains() {
        for (path, package_name) in collect_package_relative_dirs(
            <Db as salsa::DbWithJar<CowordJar>>::as_jar_db(db),
            &test_domain.src_base(),
        )
        .into_iter()
        {
            let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
            let package_path = PackagePath::new_local_or_toolchain_package(
                vfs_db,
                toolchain,
                package_name,
                &path.to_logical_path(&test_domain.src_base()),
            )
            .unwrap();
            for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path) {
                let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
                let expect_file_path = unit.determine_expect_file_path(
                    vfs_db,
                    &path.to_logical_path(&test_domain.expect_files_base()),
                    config,
                );
                std::fs::create_dir_all(expect_file_path.parent().unwrap()).unwrap();
                let output = f(db, unit);
                match std::env::var("CARGO_MANIFEST_DIR") {
                    Ok(_) => ::expect_test::expect_file![expect_file_path].assert_eq(&output),
                    Err(_) => unsafe {
                        static mut ONCE_FLAG: bool = false;
                        if !ONCE_FLAG {
                            println!(
                                "CARGO_MANIFEST_DIR not set, skip comparing with expect files"
                            );
                            ONCE_FLAG = true
                        }
                    },
                }
                if let Some(adversarials_base) = test_domain.adversarials_base() {
                    vfs_adversarial_test(
                        db,
                        &path.to_logical_path(adversarials_base),
                        unit,
                        &f,
                        config,
                    )
                }
            }
        }
    }
}
