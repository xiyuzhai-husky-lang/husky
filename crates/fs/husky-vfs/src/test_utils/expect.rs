use super::*;

pub(super) fn vfs_expect_test<'a, Db, U, R>(
    db: &'a Db,
    name: &str,
    f: &impl Fn(&'a Db, U) -> R,
    p: impl Fn(&'a Db, R) -> String,
) where
    Db: VfsTestUtils + ?Sized,
    U: VfsTestUnit,
{
    let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
    let toolchain = db.dev_toolchain().unwrap();
    for (base, out) in expect_test_base_outs() {
        std::fs::create_dir_all(&out).expect("failed_to_create_dir_all");
        for path in collect_package_relative_dirs(&base).into_iter() {
            let package_path =
                PackagePath::new_local(vfs_db, toolchain, &path.to_logical_path(&base)).unwrap();
            for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path) {
                let path = unit.decide_expect_file_path(vfs_db, name, &path.to_logical_path(&out));
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                let f = f(db, unit);
                // only test when CARGO_MANIFEST_DIR is set
                match std::env::var("CARGO_MANIFEST_DIR") {
                    Ok(_) => ::expect_test::expect_file![path].assert_eq(&p(db, f)),
                    Err(_) => (),
                }
            }
        }
    }
}
