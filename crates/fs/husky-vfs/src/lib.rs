// use timed_salsa::jar::Jar;

mod cache;
mod error;
mod file;
mod path;
mod watch;

pub use cache::HuskyFileCache;
pub use error::*;
pub use path::path_class;
pub use watch::{HasWatcherPlace, VfsWatcher, VfsWatcherThread, WatchableVfsDb};

use dashmap::{mapref::entry::Entry, DashMap};
use eyre::Context;
use file::*;
use notify_debouncer_mini::{
    notify::{RecommendedWatcher, RecursiveMode},
    Debouncer,
};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

#[timed_salsa::interned]
pub struct PathBufItd {
    #[return_ref]
    path: PathBuf,
}

#[timed_salsa::jar(db = VfsDb)]
pub struct Jar(PathBufItd, HuskyFileId, path_class);

pub trait VfsDb: timed_salsa::DbWithJar<Jar> + Vfs {
    fn file(&self, path: PathBufItd) -> VfsResult<HuskyFileId>;
    fn update_file(&mut self, path: PathBufItd) -> VfsResult<()>;
}

impl<T> VfsDb for T
where
    T: timed_salsa::DbWithJar<Jar> + Vfs,
{
    fn file(&self, path: PathBufItd) -> VfsResult<HuskyFileId> {
        match self.cache().0.entry(path.clone()) {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => Ok(*entry.get()),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                let path_ref = &path.path(self);
                self.watch_then_read(
                    path_ref,
                    Box::new(|| {
                        let content = self.read_to_string(path_ref)?;
                        Ok(*entry.insert(HuskyFileId::new(
                            self,
                            path,
                            path_class(self, path),
                            content,
                        )))
                    }),
                )
            }
        }
    }

    // todo: test this
    fn update_file(&mut self, path: PathBufItd) -> VfsResult<()> {
        let path_ref = path.path(self);
        let content = self.read_to_string(path_ref)?;
        self.file(path)?.set_content(self).to(content);
        Ok(())
    }
}

pub trait Vfs {
    fn watch_then_read<'a>(
        &self,
        path: &Path,
        read: Box<dyn FnOnce() -> VfsResult<HuskyFileId> + 'a>,
    ) -> VfsResult<HuskyFileId>;
    fn cache(&self) -> &HuskyFileCache;
    fn read_to_string(&self, path: &Path) -> VfsResult<String>;
}

#[cfg(test)]
mod tests {
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
        fn watcher_place(&mut self) -> &mut place::SingleAssignPlace<watch::VfsWatcher> {
            &mut self.watcher_place
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

        fn watch_then_read<'a>(
            &self,
            path: &Path,
            read: Box<dyn FnOnce() -> VfsResult<HuskyFileId> + 'a>,
        ) -> VfsResult<HuskyFileId> {
            read()
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
}
