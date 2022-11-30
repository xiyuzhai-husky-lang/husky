use crate::{watch::DEBOUNCE_TEST_SLEEP_TIME, *};
use crossbeam_channel::{unbounded, Sender};
use dashmap::DashMap;
use husky_package_path::PackagePathJar;
use husky_print_utils::p;
use husky_source_path::{SourcePath, SourcePathJar};
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
}

impl salsa::Database for VfsTestsDatabase {}

impl ParallelDatabase for VfsTestsDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(VfsTestsDatabase {
            storage: self.storage.snapshot(),
            watcher_place: self.watcher_place.clone(),
        })
    }
}

#[test]
fn vfs_db_works() {
    let db = VfsTestsDatabase::default();
    let db = WatchedVfs::new(db);
    let tempdir = tempfile::tempdir().unwrap();
    let somepath = tempdir.path().join("somepath");
    let somepath_itd: SourcePath = todo!();
    //  = db.apply(|db| PathBufItd::new(db, somepath.clone()));
    std::fs::write(somepath.clone(), "Hello, world!");
    let somefile = db.apply(|db| db.file(somepath_itd).unwrap());
    assert!(db.apply(|db| somefile.content(db) == "Hello, world!"),);
    std::fs::write(somepath, "Hello, world!2");
    let a = DEBOUNCE_TEST_SLEEP_TIME;
    std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
    assert!(db.apply(|db| somefile.content(db) == "Hello, world!2"))
}
