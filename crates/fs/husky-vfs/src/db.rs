use crate::*;
use husky_fs_specs::FsSpecsError;
use husky_path_utils::{collect_husky_package_dirs, derive_library_path_from_cargo_manifest_dir};
use husky_text::TextChange;
use vec_like::VecSet;

pub trait VfsDb: salsa::DbWithJar<VfsJar> + WordDb + Send + VfsDbInner {
    fn vfs_path_menu(&self, toolchain: Toolchain) -> ToolchainResult<&VfsPathMenu>;
    fn current_toolchain(&self) -> VfsResult<Toolchain>;
    fn live_packages(
        &self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>>;
    fn package_manifest_content(&self, package_path: PackagePath) -> VfsResult<&str>;
    fn module_diff_path(&self, module_path: ModulePath) -> VfsResult<DiffPath>;
    fn module_content(&self, module_path: ModulePath) -> VfsResult<&str>;
    fn package_dir(&self, package_path: PackagePath) -> &VfsResult<DiffPath>;
    fn collect_local_packages(
        &self,
        toolchain: Toolchain,
        dir: &Path,
    ) -> VfsResult<Vec<PackagePath>>;
    fn collect_crates(
        &self,
        toolchain: Toolchain,
        package_path: PackagePath,
    ) -> VfsResult<Vec<CratePath>>;
    fn collect_probable_modules(&self, package: PackagePath) -> Vec<ModulePath>;
    fn resolve_module_path(&self, toolchain: Toolchain, path: &Path) -> VfsResult<ModulePath>;
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path;
    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path;
}

// don't leak this outside the crate
pub trait VfsDbInner {
    fn file_from_diff_path(&self, path: DiffPath) -> VfsResult<File>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_cache(&self) -> &VfsCache;
    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()>;
    fn refresh_file_from_disk(&mut self, path: &Path) -> VfsResult<()>
    where
        Self: 'static;
    fn corgi_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.vfs_jar().cache().corgi_install_path()
    }
    fn huskyup_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.vfs_jar().cache().huskyup_install_path()
    }
    fn is_inside_installed_corgi_or_huskyup(&self, path: &Path) -> VfsResult<bool> {
        Ok(path.starts_with(self.corgi_install_path()?)
            || path.starts_with(self.huskyup_install_path()?))
    }
    fn calc_durability(&self, path: &Path) -> VfsResult<salsa::Durability>;
}

impl<T> VfsDbInner for T
where
    T: salsa::DbWithJar<VfsJar> + WordDb + Send + 'static,
{
    fn file_from_diff_path(&self, abs_path: DiffPath) -> VfsResult<File> {
        Ok(
            match self
                .vfs_jar()
                .cache()
                .files()
                .entry(abs_path.path(self).to_owned())
            {
                // If the file already exists in our cache then just return it.
                Entry::Occupied(entry) => *entry.get(),
                // If we haven't read this file yet set up the watch, read the
                // contents, store it in the cache, and return it.
                Entry::Vacant(entry) => {
                    let path = abs_path.path(self);
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
                        self.calc_durability(path)?,
                    ))
                }
            },
        )
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
    fn refresh_file_from_disk(&mut self, path: &Path) -> VfsResult<()>
    where
        T: 'static,
    {
        let content = read_file_content(&path);
        self.set_content(path, content)
    }

    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()> {
        let abs_path = DiffPath::try_new(self, path)?;
        let file = match self
            .vfs_jar()
            .cache()
            .files()
            .entry(abs_path.path(self).to_owned())
        {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => *entry.get(),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                let path = abs_path.path(self);
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
                    self.calc_durability(path)?,
                ))
            }
        };
        file.set_content(self).to(content);
        Ok(())
    }

    fn calc_durability(&self, path: &Path) -> VfsResult<salsa::Durability> {
        Ok(if self.is_inside_installed_corgi_or_huskyup(path)? {
            salsa::Durability::HIGH
        } else {
            salsa::Durability::LOW
        })
    }
}

impl<T> VfsDb for T
where
    T: salsa::DbWithJar<VfsJar> + WordDb + Send + 'static,
{
    fn vfs_path_menu(&self, toolchain: Toolchain) -> ToolchainResult<&VfsPathMenu> {
        Ok(vfs_path_menu(self, toolchain)
            .as_ref()
            .map_err(|e| e.clone())?)
    }

    fn package_manifest_content(&self, package_path: PackagePath) -> VfsResult<&str> {
        package_manifest_file(self, package_path)?.text(self)
    }

    fn live_packages(
        &self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>> {
        self.vfs_cache().live_packages()
    }

    fn module_diff_path(&self, module_path: ModulePath) -> VfsResult<DiffPath> {
        module_diff_path(self, module_path)
    }

    fn module_content(&self, module_path: ModulePath) -> VfsResult<&str> {
        let diff_path = module_diff_path(self, module_path)?;
        self.file_from_diff_path(diff_path)?.text(self)
    }

    fn package_dir(&self, package: PackagePath) -> &VfsResult<DiffPath> {
        package_dir(self, package)
    }

    fn collect_local_packages(
        &self,
        toolchain: Toolchain,
        dir: &Path,
    ) -> VfsResult<Vec<PackagePath>> {
        collect_husky_package_dirs(dir)
            .into_iter()
            .map(|path| PackagePath::new_local(self, toolchain, &path).map_err(|e| e.into()))
            .collect()
    }

    fn collect_crates(
        &self,
        toolchain: Toolchain,
        package_path: PackagePath,
    ) -> VfsResult<Vec<CratePath>> {
        let mut crates: Vec<CratePath> = vec![];
        let package_dir = self.package_dir(package_path).as_ref()?.path(self);
        if package_dir.join("src/lib.hsy").exists() {
            crates.push(CratePath::new(self, package_path, CrateKind::Library));
        }
        if package_dir.join("src/main.hsy").exists() {
            crates.push(CratePath::new(self, package_path, CrateKind::Main));
        }
        if package_dir.join("src/bin").exists() {
            todo!()
        }
        if package_dir.join("tests").exists() {
            todo!()
        }
        Ok(crates)
    }

    fn collect_probable_modules(&self, package: PackagePath) -> Vec<ModulePath> {
        fn collect_probable_modules(
            db: &dyn VfsDb,
            parent: ModulePath,
            dir: &Path,
            modules: &mut Vec<ModulePath>,
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
                    if let Some(ident) = path
                        .file_name()
                        .and_then(|filename| filename.to_str())
                        .and_then(|filename| db.it_ident_borrowed(filename))
                    {
                        collect_probable_modules(
                            db,
                            ModulePath::new_child(db, parent, ident),
                            &path,
                            modules,
                        )?
                    }
                } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hsy")
                {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        let push_flag = match file_stem {
                            "main" | "lib" => match parent.data(db) {
                                ModulePathData::Root(_) => false,
                                ModulePathData::Child { .. } => true,
                            },
                            _ => true,
                        };
                        if push_flag {
                            if let Some(ident) = db.it_ident_borrowed(file_stem) {
                                modules.push(ModulePath::new_child(db, parent, ident))
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        let mut modules = vec![];
        let Ok(diff_path) = &self.package_dir(package).as_ref() else {
            return vec![]
        };
        let package_dir = diff_path.data(self);
        if package_dir.join("src/lib.hsy").exists() {
            let root_module =
                ModulePath::new_root(self, CratePath::new(self, package, CrateKind::Library));
            modules.push(root_module);
            collect_probable_modules(self, root_module, &package_dir.join("src"), &mut modules);
            if package_dir.join("src/main.hsy").exists() {
                todo!()
            }
            if package_dir.join("src/bin").exists() {
                todo!()
            }
        } else if package_dir.join("src/main.hsy").exists() {
            let root_module =
                ModulePath::new_root(self, CratePath::new(self, package, CrateKind::Main));
            modules.push(root_module);
            collect_probable_modules(self, root_module, &package_dir.join("src"), &mut modules);
            if package_dir.join("src/bin").exists() {
                todo!()
            }
        }
        modules
    }

    fn resolve_module_path(&self, toolchain: Toolchain, path: &Path) -> VfsResult<ModulePath> {
        let module_path = resolve_module_path(self, toolchain, path)?;
        let package_path = module_path.package_path(self);
        self.vfs_cache().add_live_package(package_path);
        Ok(module_path)
    }

    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path {
        match toolchain.data(self) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path } => library_path.data(self),
        }
    }

    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path {
        published_toolchain_library_path(self, toolchain)
    }

    fn current_toolchain(&self) -> VfsResult<Toolchain> {
        current_toolchain(self)
    }
}

fn read_file_content(path: &Path) -> FileContent {
    if !path.exists() {
        FileContent::NotExists
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
