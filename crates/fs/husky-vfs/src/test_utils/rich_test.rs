use salsa::DebugWithDb;

use super::*;
use std::collections::HashMap;

/// includes adversarial
pub(super) fn vfs_rich_test<DB, U>(f: impl Fn(&::salsa::Db, U) -> String, config: &VfsTestConfig)
where
    DB: VfsTestUtils,
    U: IsVfsTestUnit + salsa::DebugWithDb,
{
    // use this to detect conflicting paths
    let mut paths_used: HashMap<PathBuf, PathUsage<U>> = Default::default();
    for test_domain in config.test_domains() {
        for package_relative_dir in
            collect_package_relative_dirs(&test_domain.src_base()).into_iter()
        {
            let db = &mut *DB::default();
            let toolchain = db.dev_toolchain().unwrap();
            let package_path = PackagePath::new_local_or_toolchain_package(
                db,
                toolchain,
                &package_relative_dir.to_logical_path(&test_domain.src_base()),
            )
            .unwrap();
            let units = collect_units_from_package_path::<U>(db, package_path);
            for unit in units {
                let expect_file_path = unit.determine_expect_file_path(
                    db,
                    &package_relative_dir.to_logical_path(&test_domain.expect_files_base()),
                    config,
                );
                if let Some(old_usage) =
                    paths_used.insert(expect_file_path.clone(), PathUsage::Expect(unit))
                {
                    panic!(
                        r#"Detect conflicting paths for unit `{:?}` while doing expect testing!
Old usage is `{:?}`.
The conflicting path is `{expect_file_path:?}`"#,
                        unit.debug(db),
                        old_usage.debug(db),
                    )
                }
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
                    let adversarial_path = package_relative_dir.to_logical_path(adversarials_base);
                    vfs_adversarial_test(db, &adversarial_path, unit, &f, config, &mut paths_used)
                }
            }
        }
    }
}
