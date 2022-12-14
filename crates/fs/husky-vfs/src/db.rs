use crate::*;
use husky_entity_path::{CratePathKind, EntityPath, EntityPathData};
use husky_package_path::{PackagePath, PackagePathData};
use husky_source_path::{SourcePath, SourcePathData};

pub trait VfsDb: salsa::DbWithJar<VfsJar> + SourcePathDb + Send {
    fn file_content(&self, path: SourcePath) -> VfsResult<&str>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()>;
    fn package_dir(&self, package: PackagePath) -> VfsResult<PathBuf> {
        match self.package_path_data(package) {
            PackagePathData::Builtin { ident, toolchain } => todo!(),
            PackagePathData::Global { version } => todo!(),
            PackagePathData::Local(path) => Ok(path.to_owned()),
            PackagePathData::Git(_) => todo!(),
        }
    }
    fn all_possible_modules(&self, package: PackagePath) -> VfsResult<Vec<EntityPath>>;
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

    fn all_possible_modules(&self, package: PackagePath) -> VfsResult<Vec<EntityPath>> {
        fn collect_all_possible_modules(
            db: &dyn VfsDb,
            parent: EntityPath,
            dir: &Path,
            modules: &mut Vec<EntityPath>,
        ) -> VfsResult<()> {
            let read_dir =
                std::fs::read_dir(dir).map_err(|e| VfsError::new_io_error(dir.to_owned(), e))?;
            let mut paths: Vec<PathBuf> = read_dir
                .map(|entry| -> VfsResult<PathBuf> {
                    let entry = entry.map_err(|e| VfsError::new_io_error(dir.to_owned(), e))?;
                    Ok(entry.path())
                })
                .collect::<VfsResult<_>>()?;
            // sort is important for testing
            paths.sort();
            for path in paths {
                if path.is_dir() {
                    let Some(filename) = path
                        .file_name() else { continue };
                    let Some(filename) = filename.to_str() else { continue };
                    let Some(ident) = db.it_ident_borrowed(filename) else { continue };
                    let new_parent = db.it_entity_path(EntityPathData::Childpath { parent, ident });
                    collect_all_possible_modules(db, new_parent, &path, modules)?
                } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hsy")
                {
                    let Some(ident) = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .and_then(|s| db.it_ident_borrowed(s))
                        else { continue };
                    modules.push(db.it_entity_path(EntityPathData::Childpath { parent, ident }))
                }
            }
            Ok(())
        }

        let mut modules = vec![];
        let package_dir = self.package_dir(package)?;
        if package_dir.join("src/lib.hsy").exists() {
            let root_module = self.it_entity_path(EntityPathData::Crate {
                package,
                kind: CratePathKind::Library,
            });
            modules.push(root_module);
            collect_all_possible_modules(self, root_module, &package_dir.join("src"), &mut modules);
            if package_dir.join("src/main.hsy").exists() {
                todo!()
            }
            if package_dir.join("src/bin").exists() {
                todo!()
            }
        } else if package_dir.join("src/main.hsy").exists() {
            let root_module = self.it_entity_path(EntityPathData::Crate {
                package,
                kind: CratePathKind::Main,
            });
            modules.push(root_module);
            collect_all_possible_modules(self, root_module, &package_dir.join("src"), &mut modules);
            if package_dir.join("src/bin").exists() {
                todo!()
            }
        }
        Ok(modules)
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
