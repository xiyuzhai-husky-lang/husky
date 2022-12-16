use crate::{
    watch::{WatchedVfs, DEBOUNCE_TEST_SLEEP_TIME},
    *,
};
use crossbeam_channel::{unbounded, Sender};
use dashmap::DashMap;
use husky_absolute_path::AbsolutePath;
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_package_path::{PackagePathData, PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use place::SingleAssignPlace;
use replace_with::replace_with;
use salsa::Durability;
use std::{
    collections::HashSet,
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
};

#[salsa::db(VfsJar, WordJar, ToolchainJar, PackagePathJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
    watcher_place: SingleAssignPlace<VfsWatcher>,
    vfs_config: VfsConfigMimic,
}

impl salsa::Database for DB {}

impl HasVfsConfig for DB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.vfs_config
    }
}

impl ParallelDatabase for DB {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(DB {
            storage: self.storage.snapshot(),
            watcher_place: self.watcher_place.clone(),
            vfs_config: self.vfs_config.clone(),
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
        assert!(db.query(
            |db| db.file_from_absolute_path(&abs_path).content(db.deref())
                == &FileContent::OnDisk("Hello, world!".to_owned())
        ),);
        std::fs::write(&path, "Hello, world!2");
        let a = DEBOUNCE_TEST_SLEEP_TIME;
        std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
        assert!(db.query(
            |db| db.file_from_absolute_path(&abs_path).content(db.deref())
                == &FileContent::OnDisk("Hello, world!2".to_owned())
        ))
    }
}
