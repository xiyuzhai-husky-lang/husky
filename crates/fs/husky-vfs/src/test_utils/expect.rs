use super::*;

pub(super) fn vfs_expect_test<Db, U>(db: &mut Db, name: &str, f: impl Fn(&Db, U) -> String)
where
    Db: VfsTestUtils + ?Sized,
    U: VfsTestUnit,
{
    let toolchain = db.dev_toolchain().unwrap();
    for (base, out) in expect_test_base_outs() {
        std::fs::create_dir_all(&out).expect("failed_to_create_dir_all");
        for path in collect_package_relative_dirs(&base).into_iter() {
            let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
            let package_path =
                PackagePath::new_local(vfs_db, toolchain, &path.to_logical_path(&base)).unwrap();
            for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path) {
                let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
                let path = unit.decide_expect_file_path(vfs_db, name, &path.to_logical_path(&out));
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                // only test when CARGO_MANIFEST_DIR is set
                match std::env::var("CARGO_MANIFEST_DIR") {
                    Ok(_) => ::expect_test::expect_file![path].assert_eq(&f(db, unit)),
                    Err(_) => (),
                }
                vfs_robustness_test(db, &f)
            }
        }
    }
}
