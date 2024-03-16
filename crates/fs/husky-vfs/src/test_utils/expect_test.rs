use super::*;

pub(super) fn vfs_expect_test<DB, U>(f: impl Fn(&::salsa::Db, U) -> String, config: &VfsTestConfig)
where
    DB: VfsTestUtils,
    U: VfsTestUnit + salsa::DebugWithDb,
{
    for test_domain in config.test_domains() {
        for path in collect_package_relative_dirs(&test_domain.src_base()).into_iter() {
            let db = &mut *DB::default();
            let toolchain = db.dev_toolchain().unwrap();
            let package_path = PackagePath::new_local_or_toolchain_package(
                db,
                toolchain,
                &path.to_logical_path(&test_domain.src_base()),
            )
            .unwrap();
            for unit in <U as VfsTestUnit>::collect_from_package_path(db, package_path) {
                let expect_file_path = unit.determine_expect_file_path(
                    db,
                    &path.to_logical_path(&test_domain.expect_files_base()),
                    config,
                );
                std::fs::create_dir_all(expect_file_path.parent().unwrap()).unwrap();
                let output = f(db, unit);
                let output = match config.expect_file_extension() {
                    FileExtensionConfig::Markdown => format!("```rust\n{output}\n```"),
                    FileExtensionConfig::Rust => output,
                };
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
