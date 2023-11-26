use crate::*;

use husky_fs_specs::FsSpecsError;
use husky_path_utils::collect_husky_package_dirs;
use salsa::Db;
use vec_like::VecSet;
pub trait VfsDb {
    fn vfs_path_menu(&self, toolchain: Toolchain) -> &VfsPathMenu;
    fn current_toolchain(&self) -> VfsResult<Toolchain>;
    fn live_packages(
        &self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>>;
    fn collect_local_packages(
        &self,
        toolchain: Toolchain,
        dir: &Path,
    ) -> VfsResult<Vec<PackagePath>>;
    fn collect_crates(&self, package_path: PackagePath) -> VfsResult<Vec<CratePath>>;
    fn collect_probable_modules(&self, package_path: PackagePath) -> Vec<ModulePath>;
    fn resolve_module_path(&self, toolchain: Toolchain, path: &Path) -> VfsResult<ModulePath>;
    fn published_toolchain_library_path(&self, toolchain: PublishedToolchain) -> &Path;
}
// don't leak this outside the crate
pub trait VfsDbInner {
    fn file_from_virtual_path(&self, path: VirtualPath) -> VfsResult<File>;
    fn vfs_jar(&self) -> &VfsJar;
    fn vfs_jar_mut(&mut self) -> &mut VfsJar;
    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db;
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

impl VfsDbInner for Db {
    fn file_from_virtual_path(&self, abs_path: VirtualPath) -> VfsResult<File> {
        Ok(
            match self
                .vfs_jar()
                .cache()
                .files()
                .entry(abs_path.data(self).to_owned())
            {
                // If the file already exists in our cache then just return it.
                Entry::Occupied(entry) => *entry.get(),
                // If we haven't read this file yet set up the watch, read the
                // contents, store it in the cache, and return it.
                Entry::Vacant(entry) => {
                    let path = abs_path.data(self);
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
        self.jar::<VfsJar>().0
    }

    fn vfs_jar_mut(&mut self) -> &mut VfsJar {
        self.jar_mut::<VfsJar>().0
    }

    fn vfs_cache(&self) -> &VfsCache {
        self.vfs_jar().cache()
    }

    // todo: test this
    fn refresh_file_from_disk(&mut self, path: &Path) -> VfsResult<()>
    where
        Db: 'static,
    {
        let content = read_file_content(&path);
        self.set_content(path, content)
    }

    fn set_content(&mut self, path: &Path, content: FileContent) -> VfsResult<()> {
        let abs_path = VirtualPath::try_new(self, path)?;
        let file = match self
            .vfs_jar()
            .cache()
            .files()
            .entry(abs_path.data(self).to_owned())
        {
            // If the file already exists in our cache then just return it.
            Entry::Occupied(entry) => *entry.get(),
            // If we haven't read this file yet set up the watch, read the
            // contents, store it in the cache, and return it.
            Entry::Vacant(entry) => {
                let path = abs_path.data(self);
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

    fn vfs_db_mut(&mut self) -> &mut ::salsa::Db {
        self
    }
}

impl VfsDb for Db {
    fn vfs_path_menu(&self, toolchain: Toolchain) -> &VfsPathMenu {
        vfs_path_menu(self, toolchain)
    }

    fn live_packages(
        &self,
    ) -> std::sync::LockResult<std::sync::RwLockReadGuard<'_, VecSet<PackagePath>>> {
        self.vfs_cache().live_packages()
    }

    fn collect_local_packages(
        &self,
        toolchain: Toolchain,
        dir: &Path,
    ) -> VfsResult<Vec<PackagePath>> {
        collect_husky_package_dirs(self, dir)
            .into_iter()
            .map(|(path, name)| {
                PackagePath::new_local_or_toolchain_package(self, toolchain, name, &path)
                    .map_err(|e| e.into())
            })
            .collect()
    }

    fn collect_crates(&self, package_path: PackagePath) -> VfsResult<Vec<CratePath>> {
        let mut crates: Vec<CratePath> = vec![];
        let package_dir = package_path.dir(self).as_ref()?.data(self);
        if package_dir.join("src/lib.hsy").exists() {
            crates.push(CratePath::new(package_path, CrateKind::Lib, self)?);
        }
        if package_dir.join("src/main.hsy").exists() {
            crates.push(CratePath::new(package_path, CrateKind::Main, self)?);
        }
        if package_dir.join("src/bin").exists() {
            todo!()
        }
        if package_dir.join("tests").exists() {
            todo!()
        }
        Ok(crates)
    }

    /// todo: should return not only ModulePath but also files with extension "hsy" but not included in any tree
    /// so the type should be ProbableModulePath maybe
    fn collect_probable_modules(&self, package: PackagePath) -> Vec<ModulePath> {
        fn collect_probable_modules(
            db: &::salsa::Db,
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
                        .and_then(|filename| Ident::from_ref(db, filename))
                    {
                        if let Ok(child) = ModulePath::new_child(db, parent, ident) {
                            collect_probable_modules(db, child.inner(), &path, modules)?
                        }
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
                            if let Some(ident) = Ident::from_ref(db, file_stem) {
                                if let Ok(new_child) = ModulePath::new_child(db, parent, ident) {
                                    modules.push(new_child.into())
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        let mut modules = vec![];
        let Ok(diff_path) = package.dir(self) else {
            return vec![];
        };
        let package_dir = diff_path.data(self);
        if package_dir.join("src/lib.hsy").exists() {
            if let Ok(root_module) = ModulePath::new_root(
                self,
                CratePath::new(package, CrateKind::Lib, self).expect("should be valid"),
            ) {
                modules.push(root_module);
                collect_probable_modules(self, root_module, &package_dir.join("src"), &mut modules)
                    .unwrap();
                if package_dir.join("src/main.hsy").exists() {
                    todo!()
                }
                if package_dir.join("src/bin").exists() {
                    todo!()
                }
            }
        } else if package_dir.join("src/main.hsy").exists() {
            if let Ok(root_module) = ModulePath::new_root(
                self,
                CratePath::new(package, CrateKind::Main, self).expect("should be valid"),
            ) {
                modules.push(root_module);
                collect_probable_modules(self, root_module, &package_dir.join("src"), &mut modules)
                    .unwrap();
                if package_dir.join("src/bin").exists() {
                    todo!()
                }
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

#[salsa::jar(db = VfsDb)]
pub struct VfsJar(
    VfsCache,
    crate::path::workspace_path::WorkspacePath,
    crate::linktime_target_path::LinktimeTargetPath,
    crate::linktime_target_path::linktime_target_rust_dir,
    crate::path::package_path::PackagePath,
    crate::path::crate_path::package_crate_paths,
    crate::path::module_path::relative_path::module_relative_path,
    crate::path::module_path::relative_path::module_relative_stem,
    CratePath,
    ModulePath,
    module_ancestry,
    vfs_path_menu,
    VirtualPath,
    File,
    package_dir,
    package_manifest_path,
    module_virtual_path,
    package_manifest_file,
    module_file,
    Toolchain,
    PublishedToolchain,
    published_toolchain_library_path,
    current_toolchain,
    crate::snippet::Snippet,
);

impl salsa::storage::IngredientsFor for VfsCache {
    type Jar = VfsJar;

    type Ingredients = Self;

    fn create_ingredients(_routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        Default::default()
    }
}

impl VfsJar {
    pub(crate) fn cache(&self) -> &VfsCache {
        &self.0
    }

    // pub(crate) fn set_watcher(&mut self, watcher: VfsWatcher) {
    //     self.0.set_watcher(watcher)
    // }
}
