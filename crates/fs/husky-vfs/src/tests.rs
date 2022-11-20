use crate::*;
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
    file_contents: DashMap<PathBuf, String>,
    watcher_place: SingleAssignPlace<VfsWatcher>,
}

impl VfsTestsDatabase {
    fn write_file(&mut self, path: PathBufItd, new_content: String) -> VfsResult<()> {
        let pathbuf = path.path(self).to_owned();
        self.file_contents.insert(pathbuf, new_content);
        if self.cache.0.contains_key(&path) {
            self.update_file(path)?
        }
        Ok(())
    }
}

impl timed_salsa::Database for VfsTestsDatabase {}

impl HasWatcherPlace for VfsTestsDatabase {
    fn watcher_place_mut(&mut self) -> &mut place::SingleAssignPlace<watch::VfsWatcher> {
        &mut self.watcher_place
    }

    fn watcher_place(&self) -> &SingleAssignPlace<VfsWatcher> {
        &self.watcher_place
    }
}

impl Vfs for VfsTestsDatabase {
    fn read_to_string(&self, path: &Path) -> VfsResult<String> {
        if let Some(content) = self.file_contents.get_mut(path) {
            Ok(content.clone())
        } else {
            Err(VfsError::FileNotFound(path.to_owned()))
        }
    }

    fn cache(&self) -> &HuskyFileCache {
        &self.cache
    }
}

#[test]
fn vfs_db_works() {
    let mut db = VfsTestsDatabase::default();
    let somepath = PathBufItd::new(&db, "somepath".into());
    assert!(db.file(somepath).is_err());
    db.write_file(somepath, "bob is cool".to_string())
        .expect("true");
    let file = db.file(somepath).unwrap();
    assert_eq!(file.content(&db), "bob is cool");
    db.write_file(somepath, "alice is cool".to_string())
        .expect("true");
    let file = db.file(somepath).unwrap();
    assert_eq!(file.content(&db), "alice is cool");
}

#[test]
fn vfs_db_watcher_works() {
    let db = Arc::new(Mutex::new(VfsTestsDatabase::default()));
    let watcher_thread = VfsWatcherThread::new(db.clone());
    // let somepath = PathBufItd::new(&db, "somepath".into());
    // assert!(db.file(somepath).is_err());
    // db.write_file(somepath, "bob is cool".to_string())
    //     .expect("true");
    // let file = db.file(somepath).unwrap();
    // assert_eq!(file.content(&db), "bob is cool");
    // db.write_file(somepath, "alice is cool".to_string())
    //     .expect("true");
    // let file = db.file(somepath).unwrap();
    // assert_eq!(file.content(&db), "alice is cool");
}
