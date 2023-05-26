use super::*;

pub(super) fn vfs_expect_test<Db, U>(db: &mut Db, task_name: &str, f: impl Fn(&Db, U) -> String)
where
    Db: VfsTestUtils + ?Sized,
    U: VfsTestUnit,
{
    let toolchain = db.dev_toolchain().unwrap();
    for domain in vfs_test_suites() {
        for (path, package_name) in collect_package_relative_dirs(
            <Db as salsa::DbWithJar<WordJar>>::as_jar_db(db),
            &domain.src_base(),
        )
        .into_iter()
        {
            let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
            let package_path = PackagePath::new_local_package(
                vfs_db,
                toolchain,
                package_name,
                &path.to_logical_path(&domain.src_base()),
            )
            .unwrap();
            for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path) {
                let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
                let expect_file_path = unit.determine_expect_file_path(
                    vfs_db,
                    task_name,
                    &path.to_logical_path(&domain.expect_files_base()),
                );
                std::fs::create_dir_all(expect_file_path.parent().unwrap()).unwrap();
                // only test when CARGO_MANIFEST_DIR is set
                match std::env::var("CARGO_MANIFEST_DIR") {
                    Ok(_) => ::expect_test::expect_file![expect_file_path].assert_eq(&f(db, unit)),
                    Err(_) => (),
                }
                if let Some(adversarials_base) = domain.adversarials_base() {
                    vfs_adversarial_test(
                        db,
                        task_name,
                        &path.to_logical_path(adversarials_base),
                        unit,
                        &f,
                    )
                }
            }
        }
    }
}
