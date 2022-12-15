use crate::*;
use husky_absolute_path::{AbsolutePath, AbsolutePathDb};
use husky_entity_path::{CratePathKind, EntityPath, EntityPathData};
use husky_package_path::{PackagePath, PackagePathData};
use husky_source_path::{SourcePath, SourcePathData};
use husky_text::{TextChange, TextRange};

pub trait VfsDb:
    salsa::DbWithJar<VfsJar> + AbsolutePathDb + SourcePathDb + Send + VfsDbInner
{
    fn source_text(&self, path: SourcePath) -> VfsResult<&str>;
    fn package_dir(&self, package: PackagePath) -> VfsResult<PathBuf> {
        match self.package_path_data(package) {
            PackagePathData::Builtin { ident, toolchain } => todo!(),
            PackagePathData::Global { version } => todo!(),
            PackagePathData::Local(path) => Ok(path.to_owned()),
            PackagePathData::Git(_) => todo!(),
        }
    }
    fn all_possible_modules(&self, package: PackagePath) -> VfsResult<Vec<EntityPath>>;
    fn set_live_file(&mut self, path: PathBuf, text: String) -> VfsResult<()>;
    fn apply_live_file_changes(&mut self, path: PathBuf, event: Vec<TextChange>) -> VfsResult<()>;
}

// don't leak this outside the crate
pub trait VfsDbInner {
    fn source_path_from_absolute_path(&self, path: AbsolutePath) -> VfsResult<Option<SourcePath>>;
    fn absolute_path_from_source_path(&self, path: SourcePath) -> VfsResult<AbsolutePath>;
    fn file_content(&self, path: AbsolutePath, durability: salsa::Durability) -> &FileContent;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_cache(&self) -> &VfsCache;
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()>
    where
        Self: 'static;
}

impl<T> VfsDbInner for T
where
    T: salsa::DbWithJar<VfsJar> + AbsolutePathDb + SourcePathDb + Send + 'static,
{
    fn source_path_from_absolute_path(&self, path: AbsolutePath) -> VfsResult<Option<SourcePath>> {
        source_path_from_absolute_path(self, path)
    }

    fn absolute_path_from_source_path(&self, path: SourcePath) -> VfsResult<AbsolutePath> {
        absolute_path_from_source_path(self, path)
    }

    fn file_content(&self, path: AbsolutePath, durability: salsa::Durability) -> &FileContent {
        file_from_absolute_path(self, path, durability).content(self)
    }

    fn vfs_jar(&self) -> &VfsJar {
        <Self as HasJar<VfsJar>>::jar(self).0
    }

    fn vfs_jar_mut(&mut self) -> &mut VfsJar {
        <Self as HasJar<VfsJar>>::jar_mut(self).0
    }

    fn vfs_cache(&self) -> &VfsCache {
        self.vfs_jar().cache()
    }

    // todo: test this
    fn update_file(&mut self, path: PathBuf) -> VfsResult<()>
    where
        T: 'static,
    {
        let content = read_file_content(&path);
        if let path = AbsolutePath::new_from_owned(self, path)? {
            file_from_absolute_path(self, path, salsa::Durability::LOW)
                .set_content(self)
                .to(content);
        }
        Ok(())
    }
}

impl<T> VfsDb for T
where
    T: salsa::DbWithJar<VfsJar> + AbsolutePathDb + SourcePathDb + Send + 'static,
{
    fn source_text(&self, path: SourcePath) -> VfsResult<&str> {
        let abs_path = self.absolute_path_from_source_path(path)?;
        self.file_content(abs_path, path.durability(self)).text()
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

    fn set_live_file(&mut self, path: PathBuf, text: String) -> VfsResult<()> {
        let path = AbsolutePath::new_from_owned(self, path.clone())?;
        let entry = self.vfs_cache().files().entry(path);
        match entry {
            Entry::Occupied(_) => todo!(),
            Entry::Vacant(_) => todo!(),
        }
        todo!()
    }

    /// If range are omitted
    /// the new text is considered to be the full content of the document.
    fn apply_live_file_changes(&mut self, path: PathBuf, event: Vec<TextChange>) -> VfsResult<()> {
        todo!()
    }
}

fn file_from_absolute_path<T>(db: &T, abs_path: AbsolutePath, durability: salsa::Durability) -> File
where
    T: salsa::DbWithJar<VfsJar> + AbsolutePathDb + SourcePathDb + Send + 'static,
{
    match db.vfs_jar().cache().files().entry(abs_path) {
        // If the file already exists in our cache then just return it.
        Entry::Occupied(entry) => *entry.get(),
        // If we haven't read this file yet set up the watch, read the
        // contents, store it in the cache, and return it.
        Entry::Vacant(entry) => unsafe {
            let path = abs_path.path(db);
            //  &path.path(self);
            if let Some(watcher) = db.watcher() {
                let watcher = &mut watcher.0.lock().unwrap();
                watcher
                    .watcher()
                    .watch(path, RecursiveMode::NonRecursive)
                    .unwrap();
            }
            let content = read_file_content(path);
            *entry.insert(File::new(db, abs_path, content, durability))
        },
    }
}

fn read_file_content(path: &Path) -> FileContent {
    if !path.exists() {
        FileContent::NotExist
    } else if path.is_file() {
        match std::fs::read_to_string(path) {
            Ok(text) => FileContent::OnDisk(text),
            Err(e) => FileContent::Err(VfsError::new_io_error(path.to_owned(), e)),
        }
    } else if path.is_dir() {
        todo!()
    } else {
        todo!()
    }
}
