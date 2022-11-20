// use timed_salsa::jar::Jar;

mod cache;
mod error;
mod file;
mod path;
mod watcher;

pub use cache::HuskyFileCache;
pub use error::*;
pub use watcher::WatchableVfsDb;

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
pub struct Jar(PathBufItd, HuskyFileId);

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
                // Set up the watch before reading the contents to try to avoid
                // race conditions.
                // let mut watcher = self.file_watcher().lock().unwrap();
                let path_ref = &path.path(self);
                // watcher
                //     .watcher()
                //     .watch(path_ref, RecursiveMode::NonRecursive)
                //     .unwrap();
                self.watch_then_read(
                    path_ref,
                    Box::new(|| {
                        let content = self.read_to_string(path_ref)?;
                        Ok(*entry.insert(HuskyFileId::new(
                            self,
                            path,
                            HuskyFileClass::User,
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
    use replace_with::replace_with;
    use std::{collections::HashSet, sync::Mutex, time::Duration};

    #[timed_salsa::db(Jar)]
    struct VfsTestsDatabase {
        storage: timed_salsa::Storage<Self>,
        cache: HuskyFileCache,
        file_contents: DashMap<PathBuf, String>,
    }

    impl VfsTestsDatabase {
        fn new(tx: Sender<DebounceEventResult>) -> Self {
            Self {
                storage: Default::default(),
                cache: Default::default(),
                file_contents: Default::default(),
            }
        }

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
        let (tx, rx) = unbounded();
        let mut db = VfsTestsDatabase::new(tx);
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
}
