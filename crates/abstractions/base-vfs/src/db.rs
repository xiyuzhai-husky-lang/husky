use std::borrow::{Borrow, BorrowMut};

use crate::*;

use maybe_result::MaybeResult::JustOk;
use salsa::{Db, Durability};
use vec_like::VecSet;

// don't leak this outside the crate
pub trait BaseVfsDb {
    fn file_from_virtual_path(
        &self,
        path: VirtualPath,
        durability: Durability,
    ) -> BaseVfsResult<File>;
    fn vfs_jar(&self) -> &BaseVfsJar;
    fn vfs_jar_mut(&mut self) -> &mut BaseVfsJar;
    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db;
    fn vfs_cache(&self) -> &BaseVfsCache;
    fn set_content(
        &mut self,
        path: &Path,
        content: FileContent,
        durability: Durability,
    ) -> BaseVfsResult<()>;
    fn refresh_file_from_disk(&mut self, path: &Path, durability: Durability) -> BaseVfsResult<()>
    where
        Self: 'static;
}

impl BaseVfsDb for Db {
    fn file_from_virtual_path(
        &self,
        abs_path: VirtualPath,
        durability: Durability,
    ) -> BaseVfsResult<File> {
        Ok(
            match self
                .vfs_jar()
                .cache()
                .files()
                .entry(abs_path.data().to_owned())
            {
                // If the file already exists in our cache then just return it.
                Entry::Occupied(entry) => *entry.get(),
                // If we haven't read this file yet set up the watch, read the
                // contents, store it in the cache, and return it.
                Entry::Vacant(entry) => {
                    let path = abs_path.data();
                    //  &path.path(self);
                    // ad hoc
                    // if let Some(watcher) = self.watcher() {
                    //     let watcher = &mut watcher.0.lock().unwrap();
                    //     watcher
                    //         .watcher()
                    //         .watch(path, RecursiveMode::NonRecursive)
                    //         .unwrap();
                    // }
                    let content = read_file_content(path);
                    *entry.insert(File::new(
                        self.borrow(),
                        abs_path.clone(),
                        content,
                        durability,
                    ))
                }
            },
        )
    }

    fn vfs_jar(&self) -> &BaseVfsJar {
        self.jar::<BaseVfsJar>().0
    }

    fn vfs_jar_mut(&mut self) -> &mut BaseVfsJar {
        self.borrow_mut().jar_mut::<BaseVfsJar>().0
    }

    fn vfs_cache(&self) -> &BaseVfsCache {
        self.vfs_jar().cache()
    }

    // todo: test this
    fn refresh_file_from_disk(&mut self, path: &Path, durability: Durability) -> BaseVfsResult<()>
    where
        Db: 'static,
    {
        let content = read_file_content(&path);
        self.set_content(path, content, durability)
    }

    fn set_content(
        &mut self,
        path: &Path,
        content: FileContent,
        durability: Durability,
    ) -> BaseVfsResult<()> {
        let virtual_path = VirtualPath::try_new(self, path)?;
        let path = virtual_path.data();
        let file = match self
            .vfs_jar()
            .cache()
            .files()
            .entry(virtual_path.data().to_owned())
        {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => *entry.get(),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                //  &path.path(self);
                // ad hoc
                // if let Some(watcher) = self.watcher() {
                //     let watcher = &mut watcher.0.lock().unwrap();
                //     watcher
                //         .watcher()
                //         .watch(path, RecursiveMode::NonRecursive)
                //         .unwrap();
                // }
                let content = read_file_content(path);
                *entry.insert(File::new(self, virtual_path.clone(), content, durability))
            }
        };
        file.set_content(self.borrow_mut(), durability)?.to(content);
        Ok(())
    }

    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db {
        self
    }
}

fn read_file_content(path: &Path) -> FileContent {
    if !path.exists() {
        FileContent::NotExists
    } else if path.is_file() {
        match std::fs::read_to_string(path) {
            Ok(text) => FileContent::OnDisk(text),
            Err(e) => FileContent::Err(BaseVfsError::new_io_error(path.to_owned(), e)),
        }
    } else if path.is_dir() {
        // ad hoc
        FileContent::Directory(vec![])
    } else {
        todo!()
    }
}
