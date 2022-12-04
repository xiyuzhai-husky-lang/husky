use crate::*;
use husky_package_path::PackagePath;
use husky_source_path::{SourcePath, SourcePathData};

pub trait VfsDb: salsa::DbWithJar<VfsJar> + SourcePathDb + Send {
    fn file_content(&self, path: SourcePath) -> VfsResult<&str>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()>;
}

impl<T> VfsDb for T
where
    T: salsa::DbWithJar<VfsJar> + SourcePathDb + Send + 'static,
{
    fn vfs_jar(&self) -> &VfsJar {
        <Self as HasJar<VfsJar>>::jar(self).0
    }

    fn vfs_jar_mut(&mut self) -> &mut VfsJar {
        <Self as HasJar<VfsJar>>::jar_mut(self).0
    }

    fn file_content(&self, path: SourcePath) -> VfsResult<&str> {
        Ok(source_file(self, path)?.content(self))
    }

    // todo: test this
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()> {
        let content = read_to_string(&path)?;
        if let Some(path) = self.source_path_from_physical_path(&path)? {
            source_file(self, path)?.set_content(self).to(content);
        }
        Ok(())
    }
}

fn source_file<T>(db: &T, path: SourcePath) -> VfsResult<SourceFile>
where
    T: salsa::DbWithJar<VfsJar> + SourcePathDb + Send + 'static,
{
    Ok(match db.vfs_jar().husky_file_cache().data().entry(path) {
        // If the file already exists in our cache then just return it.
        Entry::Occupied(entry) => *entry.get(),
        // If we haven't read this file yet set up the watch, read the
        // contents, store it in the cache, and return it.
        Entry::Vacant(entry) => {
            let physical_path = db.source_absolute_path(path)?;
            //  &path.path(self);
            if let Some(watcher) = db.watcher() {
                let watcher = &mut watcher.0.lock().unwrap();
                watcher
                    .watcher()
                    .watch(&physical_path, RecursiveMode::NonRecursive)
                    .unwrap();
            }
            let content = read_to_string(&physical_path)?;
            *entry.insert(SourceFile::new(db, path, content))
        }
    })
}

fn read_to_string(path: &Path) -> VfsResult<String> {
    std::fs::read_to_string(path).map_err(|e| VfsError::new_io_error(path.to_owned(), e))
}
