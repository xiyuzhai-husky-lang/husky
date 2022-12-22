use crate::*;
use husky_fs_specs::FsSpecsError;
use husky_path_utils::{collect_husky_package_dirs, derive_library_path_from_cargo_manifest_dir};
use husky_text::TextChange;

pub trait VfsDb: salsa::DbWithJar<VfsJar> + WordDb + Send + VfsDbInner {
    fn path_menu(&self, toolchain: Toolchain) -> VfsResult<&PathMenu>;
    fn package_manifest_content(&self, package_path: PackagePath) -> VfsResult<&str>;
    fn module_content(&self, module_path: ModulePath) -> VfsResult<&str>;
    fn package_dir(&self, package_path: PackagePath) -> &VfsResult<AbsolutePath>;
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
    fn collect_probable_modules(&self, package_path: PackagePath) -> VfsResult<Vec<ModulePath>>;
    fn set_live_file(&mut self, path: &Path, text: String) -> VfsResult<()>;
    fn apply_live_file_changes(&mut self, path: &Path, changes: Vec<TextChange>) -> VfsResult<()>;
    fn resolve_module_path(&self, toolchain: Toolchain, path: &Path) -> VfsResult<ModulePath>;
    // toolchain
    fn lang_dev_toolchain(&self) -> Toolchain;
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path;
    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path;
}

// don't leak this outside the crate
pub trait VfsDbInner {
    fn file_from_absolute_path(&self, path: AbsolutePath) -> VfsResult<File>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_cache(&self) -> &VfsCache;
    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()>;
    fn update_file(&mut self, path: &Path) -> VfsResult<()>
    where
        Self: 'static;
    fn corgi_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.vfs_jar().cache().corgi_install_path()
    }
    fn huskyup_install_path(&self) -> Result<&PathBuf, &FsSpecsError> {
        self.vfs_jar().cache().huskyup_install_path()
    }
    fn is_inside_installed_corgi_or_huskyup(&self, path: &Path) -> VfsResult<bool> {
        assert!(path.is_absolute());
        Ok(path.starts_with(self.corgi_install_path()?)
            || path.starts_with(self.huskyup_install_path()?))
    }
    fn calc_durability(&self, path: &Path) -> VfsResult<salsa::Durability>;
}

impl<T> VfsDbInner for T
where
    T: salsa::DbWithJar<VfsJar> + WordDb + Send + 'static,
{
    fn file_from_absolute_path(&self, abs_path: AbsolutePath) -> VfsResult<File> {
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
    fn update_file(&mut self, path: &Path) -> VfsResult<()>
    where
        T: 'static,
    {
        let content = read_file_content(&path);
        self.set_content(path, content)
    }
    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()> {
        let abs_path = AbsolutePath::try_new(self, path)?;
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
    fn path_menu(&self, toolchain: Toolchain) -> VfsResult<&PathMenu> {
        Ok(path_menu(self, toolchain).as_ref()?)
    }

    fn package_manifest_content(&self, package_path: PackagePath) -> VfsResult<&str> {
        package_manifest_file(self, package_path)?.text(self)
    }

    fn module_content(&self, module_path: ModulePath) -> VfsResult<&str> {
        let abs_path = module_absolute_path(self, module_path)?;
        self.file_from_absolute_path(abs_path)?.text(self)
    }

    fn package_dir(&self, package: PackagePath) -> &VfsResult<AbsolutePath> {
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

    fn collect_probable_modules(&self, package: PackagePath) -> VfsResult<Vec<ModulePath>> {
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
        let package_dir = self.package_dir(package).as_ref()?.data(self);
        if package_dir.join("src/lib.hsy").exists() {
            let root_module =
                ModulePath::new_root(self, CratePath::new(self, package, CrateKind::Library));
            modules.push(root_module);
            collect_probable_modules(self, root_module, &package_dir.join("src"), &mut modules)?;
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
            collect_probable_modules(self, root_module, &package_dir.join("src"), &mut modules)?;
            if package_dir.join("src/bin").exists() {
                todo!()
            }
        }
        Ok(modules)
    }

    fn set_live_file(&mut self, path: &Path, text: String) -> VfsResult<()> {
        self.set_content(path, FileContent::Live(text))
    }

    /// If range are omitted
    /// the new text is considered to be the full content of the document.
    fn apply_live_file_changes(&mut self, _path: &Path, _event: Vec<TextChange>) -> VfsResult<()> {
        eprintln!("todo: apply_live_file_changes");
        Ok(())
    }

    fn resolve_module_path(&self, toolchain: Toolchain, path: &Path) -> VfsResult<ModulePath> {
        resolve_module_path(self, toolchain, path)
    }

    // toolchain
    fn lang_dev_toolchain(&self) -> Toolchain {
        let library_path = derive_library_path_from_cargo_manifest_dir();
        Toolchain::new(self, ToolchainData::Local { library_path })
    }
    fn toolchain_library_path(&self, toolchain: Toolchain) -> &Path {
        match toolchain.data(self) {
            ToolchainData::Published(_) => todo!(),
            ToolchainData::Local { library_path } => &library_path,
        }
    }

    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path {
        published_toolchain_library_path(self, toolchain)
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
