use crate::*;
use husky_package_path::PackagePath;
use husky_source_path::{SourcePath, SourcePathData};

pub trait VfsDb: salsa::DbWithJar<VfsJar> + SourcePathDb + Send {
    fn file(&self, path: PathBufItd) -> VfsResult<SourceFile>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()>;
    fn package_manifest_toml_file(&self, package: PackagePath) -> VfsResult<SourceFile>;
}

impl<T> VfsDb for T
where
    T: salsa::DbWithJar<VfsJar> + SourcePathDb + ParallelDatabase + Send + 'static,
{
    fn vfs_jar(&self) -> &VfsJar {
        <Self as HasJar<VfsJar>>::jar(self).0
    }

    fn vfs_jar_mut(&mut self) -> &mut VfsJar {
        <Self as HasJar<VfsJar>>::jar_mut(self).0
    }

    fn file(&self, path: PathBufItd) -> VfsResult<SourceFile> {
        match self.vfs_jar().husky_file_cache().data().entry(path.clone()) {
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
                let content = std::fs::read_to_string(path_ref)?;
                Ok(*entry.insert(SourceFile::new(self, path, path_class(self, path), content)))
            }
        }
    }

    // todo: test this
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()> {
        let content = std::fs::read_to_string(&path)?;
        self.file(PathBufItd::new(self, path))?
            .set_content(self)
            .to(content);
        Ok(())
    }

    fn package_manifest_toml_file(&self, package: PackagePath) -> VfsResult<SourceFile> {
        let source_path = self.it_source_path(SourcePathData::CorgiToml(package));
        self.file(todo!())
    }
}
