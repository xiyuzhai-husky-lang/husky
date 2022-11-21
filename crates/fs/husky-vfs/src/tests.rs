use crate::{watch::DEBOUNCE_TEST_SLEEP_TIME, *};
use crossbeam_channel::{unbounded, Sender};
use dashmap::DashMap;
use husky_print_utils::p;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use place::SingleAssignPlace;
use replace_with::replace_with;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    time::Duration,
};

#[timed_salsa::db(Jar)]
#[derive(Default)]
struct VfsTestsDatabase {
    storage: timed_salsa::Storage<Self>,
    cache: HuskyFileCache,
    watcher_place: SingleAssignPlace<VfsWatcher>,
}

impl timed_salsa::Database for VfsTestsDatabase {}

impl ParallelDatabase for VfsTestsDatabase {
    fn snapshot(&self) -> timed_salsa::Snapshot<Self> {
        timed_salsa::Snapshot::new(VfsTestsDatabase {
            storage: self.storage.snapshot(),
            cache: self.cache.snapshot(),
            watcher_place: self.watcher_place.clone(),
        })
    }
}

impl HasWatcherPlace for VfsTestsDatabase {
    fn watcher_place_mut(&mut self) -> &mut place::SingleAssignPlace<watch::VfsWatcher> {
        &mut self.watcher_place
    }

    fn watcher_place(&self) -> &SingleAssignPlace<VfsWatcher> {
        &self.watcher_place
    }
}

impl HasFileCache for VfsTestsDatabase {
    fn cache(&self) -> &HuskyFileCache {
        &self.cache
    }
}

#[test]
fn vfs_db_works() {
    let db = VfsTestsDatabase::default();
    let db = WatchedVfs::new(db);
    let tempdir = tempfile::tempdir().unwrap();
    let somepath = tempdir.path().join("somepath");
    let somepath_itd = db.apply(|db| PathBufItd::new(db, somepath.clone()));
    std::fs::write(somepath.clone(), "Hello, world!");
    let somefile = db.apply(|db| db.file(somepath_itd).unwrap());
    assert!(db.apply(|db| somefile.content(db) == "Hello, world!"),);
    std::fs::write(somepath, "Hello, world!2");
    let a = DEBOUNCE_TEST_SLEEP_TIME;
    std::thread::sleep(DEBOUNCE_TEST_SLEEP_TIME);
    assert!(db.apply(|db| somefile.content(db) == "Hello, world!2"))
}
