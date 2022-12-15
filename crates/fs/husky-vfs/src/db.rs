use crate::*;
use husky_absolute_path::AbsolutePath;
use husky_entity_path::{CratePathKind, EntityPath, EntityPathData, EntityPathDb};
use husky_package_path::{PackagePath, PackagePathData, PackagePathDb};
use husky_text::{TextChange, TextRange};

pub trait VfsDb:
    salsa::DbWithJar<VfsJar> + HasVfsConfig + EntityPathDb + PackagePathDb + Send + VfsDbInner
{
    fn package_manifest_content(&self, package_path: PackagePath) -> VfsResult<&str>;
    fn module_content(&self, entity_path: EntityPath) -> VfsResult<&str>;
    fn package_dir(&self, package: PackagePath) -> &VfsResult<AbsolutePath>;
    fn all_possible_modules(&self, package: PackagePath) -> VfsResult<Vec<EntityPath>>;
    fn set_live_file(&mut self, path: &Path, text: String) -> VfsResult<()>;
    fn apply_live_file_changes(&mut self, path: &Path, event: Vec<TextChange>) -> VfsResult<()>;
}

// don't leak this outside the crate
pub trait VfsDbInner {
    fn file_from_absolute_path(&self, path: &AbsolutePath) -> File;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_cache(&self) -> &VfsCache;
    fn update_file(&mut self, path: &Path) -> VfsResult<()>
    where
        Self: 'static;
    fn calc_durability(&self, path: &Path) -> salsa::Durability;
}

impl<T> VfsDbInner for T
where
    T: salsa::DbWithJar<VfsJar> + HasVfsConfig + EntityPathDb + PackagePathDb + Send + 'static,
{
    fn file_from_absolute_path(&self, abs_path: &AbsolutePath) -> File {
        match self
            .vfs_jar()
            .cache()
            .files()
            .entry(abs_path.path().to_owned())
        {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => *entry.get(),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => unsafe {
                let path = abs_path.path();
                //  &path.path(self);
                if let Some(watcher) = self.watcher() {
                    let watcher = &mut watcher.0.lock().unwrap();
                    watcher
                        .watcher()
                        .watch(path, RecursiveMode::NonRecursive)
                        .unwrap();
                }
                let content = read_file_content(path);
                *entry.insert(File::new(
                    self,
                    abs_path.clone(),
                    content,
                    self.calc_durability(path),
                ))
            },
        }
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
    fn update_file(&mut self, path: &Path) -> VfsResult<()>
    where
        T: 'static,
    {
        let content = read_file_content(&path);
        if let abs_path = AbsolutePath::new(&path)? {
            let abs_path = &abs_path;
            let file = match self
                .vfs_jar()
                .cache()
                .files()
                .entry(abs_path.path().to_owned())
            {
                // If the file already exists in our cache then just return it.
                Entry::Occupied(entry) => *entry.get(),
                // If we haven't read this file yet set up the watch, read the
                // contents, store it in the cache, and return it.
                Entry::Vacant(entry) => unsafe {
                    let path = abs_path.path();
                    //  &path.path(self);
                    if let Some(watcher) = self.watcher() {
                        let watcher = &mut watcher.0.lock().unwrap();
                        watcher
                            .watcher()
                            .watch(path, RecursiveMode::NonRecursive)
                            .unwrap();
                    }
                    let content = read_file_content(path);
                    *entry.insert(File::new(
                        self,
                        abs_path.clone(),
                        content,
                        self.calc_durability(path),
                    ))
                },
            };
            file.set_content(self).to(content);
        }
        Ok(())
    }

    fn calc_durability(&self, path: &Path) -> salsa::Durability {
        if let Some(corgi_install_path) = self.vfs_config().corgi_install_path() {
            todo!()
        } else {
            salsa::Durability::LOW
        }
    }
}

impl<T> VfsDb for T
where
    T: salsa::DbWithJar<VfsJar> + HasVfsConfig + EntityPathDb + PackagePathDb + Send + 'static,
{
    fn package_manifest_content(&self, package_path: PackagePath) -> VfsResult<&str> {
        package_manifest_file(self, package_path)?.text(self)
    }

    fn module_content(&self, entity_path: EntityPath) -> VfsResult<&str> {
        let abs_path = module_path(self, entity_path).as_ref()?;
        self.file_from_absolute_path(abs_path).text(self)
    }

    fn package_dir(&self, package: PackagePath) -> &VfsResult<AbsolutePath> {
        package_dir(self, package)
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
        let package_dir = self.package_dir(package).as_ref()?;
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

    fn set_live_file(&mut self, path: &Path, text: String) -> VfsResult<()> {
        let abs_path = AbsolutePath::new(path)?;
        // ad hoc
        let entry = self.vfs_cache().files().entry(abs_path.path().to_owned());
        match entry {
            Entry::Occupied(_) => todo!(),
            Entry::Vacant(_) => todo!(),
        }
        todo!()
    }

    /// If range are omitted
    /// the new text is considered to be the full content of the document.
    fn apply_live_file_changes(&mut self, path: &Path, event: Vec<TextChange>) -> VfsResult<()> {
        todo!()
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
