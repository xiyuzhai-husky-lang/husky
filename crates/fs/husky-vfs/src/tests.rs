use crate::{
    watch::{WatchedVfs, DEBOUNCE_TEST_SLEEP_TIME},
    *,
};
use crossbeam_channel::{unbounded, Sender};
use dashmap::DashMap;
use husky_package_path::{PackagePathData, PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_source_path::{
    HasSourcePathConfig, SourcePath, SourcePathConfig, SourcePathConfigMimic, SourcePathData,
    SourcePathJar,
};
use husky_toolchain::ToolchainJar;
use husky_word::WordJar;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use place::SingleAssignPlace;
use replace_with::replace_with;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

#[salsa::db(VfsJar, WordJar, ToolchainJar, PackagePathJar, SourcePathJar)]
#[derive(Default)]
struct VfsTestsDatabase {
    storage: salsa::Storage<Self>,
    watcher_place: SingleAssignPlace<VfsWatcher>,
    vfs_config: SourcePathConfigMimic,
}

impl salsa::Database for VfsTestsDatabase {}

impl HasSourcePathConfig for VfsTestsDatabase {
    fn source_path_config(&self) -> &SourcePathConfig {
        &self.vfs_config
    }
}

impl ParallelDatabase for VfsTestsDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(VfsTestsDatabase {
            storage: self.storage.snapshot(),
            watcher_place: self.watcher_place.clone(),
            vfs_config: self.vfs_config.clone(),
        })
    }
}

#[test]
fn vfs_db_works() {
    let db = VfsTestsDatabase::default();
    let db = WatchedVfs::new(db);
    let tempdir = tempfile::tempdir().unwrap();
    let some_pkg_dir = tempdir.path().join("somepath");
    std::fs::create_dir(&some_pkg_dir).unwrap();
    let corgi_toml_path: SourcePath = db.apply(|db| {
        db.it_corgi_toml_path(db.it_package_path(PackagePathData::Local(some_pkg_dir)))
    });
    let corgi_toml_physical_path = db
        .apply(|db| db.source_to_absolute_path(corgi_toml_path))
        .unwrap();
    p!(corgi_toml_physical_path);
    std::fs::write(&corgi_toml_physical_path, "Hello, world!");
    let somefile = db.apply(|db| db.file(corgi_toml_path).unwrap());
    assert!(db.apply(|db| somefile.content(db) == "Hello, world!"),);
    std::fs::write(&corgi_toml_physical_path, "Hello, world!2");
    let a = DEBOUNCE_TEST_SLEEP_TIME;
    std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
    assert!(db.apply(|db| somefile.content(db) == "Hello, world!2"))
}
