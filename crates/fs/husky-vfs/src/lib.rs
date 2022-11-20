// use timed_salsa::jar::Jar;

mod cache;
mod error;
mod file;
mod path;
#[cfg(test)]
mod tests;
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

pub trait VfsDb: timed_salsa::DbWithJar<Jar> + Vfs + HasWatcherPlace {
    fn file(&self, path: PathBufItd) -> VfsResult<HuskyFileId>;
    fn update_file(&mut self, path: PathBufItd) -> VfsResult<()>;
}

impl<T> VfsDb for T
where
    T: timed_salsa::DbWithJar<Jar> + Vfs + HasWatcherPlace + Send + 'static,
{
    fn file(&self, path: PathBufItd) -> VfsResult<HuskyFileId> {
        match self.cache().0.entry(path.clone()) {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => Ok(*entry.get()),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                let path_ref = &path.path(self);
                let watcher = &mut *self
                    .watcher()
                    .expect("watcher is not set")
                    .0
                    .lock()
                    .unwrap();
                watcher
                    .watcher()
                    .watch(path_ref, RecursiveMode::NonRecursive)
                    .unwrap();
                let content = self.read_to_string(path_ref)?;
                Ok(*entry.insert(HuskyFileId::new(
                    self,
                    path,
                    path_class(self, path),
                    content,
                )))
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
    fn cache(&self) -> &HuskyFileCache;
    fn read_to_string(&self, path: &Path) -> VfsResult<String>;
}
