use crate::{
    watch::{WatchedVfs, DEBOUNCE_TEST_SLEEP_TIME},
    *,
};
use husky_absolute_path::AbsolutePath;
use husky_entity_path::EntityPathJar;
use husky_package_path::PackagePathJar;
use husky_toolchain::*;
use husky_word::WordJar;
use std::ops::Deref;

#[salsa::db(VfsJar, WordJar, ToolchainJar, PackagePathJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(DB {
            storage: self.storage.snapshot(),
        })
    }
}

#[test]
fn watcher_works() {
    let db = DB::default();
    let db = WatchedVfs::new(db);
    let tempdir = tempfile::tempdir().unwrap();
    let some_pkg_dir = tempdir.path().join("somepath");
    std::fs::create_dir(&some_pkg_dir).unwrap();
    let path = some_pkg_dir.join("Corgi.toml");
    let abs_path: AbsolutePath = AbsolutePath::new(&path).unwrap();
    unsafe {
        std::fs::write(&path, "Hello, world!");
        assert!(db.query(|db| db
            .file_from_absolute_path(&abs_path)
            .unwrap()
            .content(db.deref())
            == &FileContent::OnDisk("Hello, world!".to_owned())),);
        std::fs::write(&path, "Hello, world!2");
        let _a = DEBOUNCE_TEST_SLEEP_TIME;
        std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
        assert!(db.query(|db| db
            .file_from_absolute_path(&abs_path)
            .unwrap()
            .content(db.deref())
            == &FileContent::OnDisk("Hello, world!2".to_owned())))
    }
}
