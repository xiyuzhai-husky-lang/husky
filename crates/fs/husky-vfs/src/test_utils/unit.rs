use super::*;

pub trait IsVfsTestUnit: Copy {
    fn collect_from_package_path(db: &::salsa::Db, package_path: PackagePath) -> Vec<Self>;
    fn determine_expect_file_path(
        &self,
        db: &::salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf;

    fn determine_adversarial_path(
        self,
        db: &::salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf>;
    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath>;
}

impl IsVfsTestUnit for PackagePath {
    fn collect_from_package_path(_db: &::salsa::Db, package_path: PackagePath) -> Vec<Self> {
        vec![package_path]
    }

    fn determine_expect_file_path(
        &self,
        _db: &::salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        package_expect_files_dir
            .join(config.test_name())
            .with_extension(config.expect_file_extension().str())
    }

    fn determine_adversarial_path(
        self,
        _db: &::salsa::Db,
        _adversarial_kind: AdversarialKind,
        _package_adversarials_dir: &Path,
        _config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        None
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}

impl IsVfsTestUnit for CratePath {
    fn collect_from_package_path(db: &::salsa::Db, package_path: PackagePath) -> Vec<Self> {
        db.collect_crates(package_path).unwrap_or_default()
    }

    fn determine_expect_file_path(
        &self,
        db: &::salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        package_expect_files_dir.join(format!(
            "{}/{}.{}",
            config.test_name(),
            match self.kind(db) {
                CrateKind::Lib => format!("src/lib"),
                CrateKind::Main => format!("src/main"),
                CrateKind::Bin(_) => todo!(),
                CrateKind::IntegratedTest(_) => todo!(),
                CrateKind::Example => todo!(),
                CrateKind::Task => format!("task"),
                CrateKind::Requirements => format!("requirements"),
            },
            config.expect_file_extension().str()
        ))
    }

    fn determine_adversarial_path(
        self,
        _db: &::salsa::Db,
        _adversarial_kind: AdversarialKind,
        _package_adversarials_dir: &Path,
        _config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        None
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}

impl IsVfsTestUnit for ModulePath {
    fn collect_from_package_path(db: &::salsa::Db, package_path: PackagePath) -> Vec<Self> {
        db.collect_probable_modules(package_path)
    }

    fn determine_expect_file_path(
        &self,
        db: &::salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        let path_parent_dir =
            determine_expect_file_path_parent_dir(db, *self, package_expect_files_dir, config);
        match self.data(db) {
            ModulePathData::Root(crate_path) => path_parent_dir.join(format!(
                "{}.{}",
                match crate_path.kind(db) {
                    CrateKind::Lib => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Bin(_) => todo!(),
                    CrateKind::IntegratedTest(_) => todo!(),
                    CrateKind::Example => todo!(),
                    CrateKind::Task => todo!(),
                    CrateKind::Requirements => todo!(),
                },
                config.expect_file_extension().str()
            )),
            ModulePathData::Child { .. } => {
                path_parent_dir.with_extension(config.expect_file_extension().str())
            }
            ModulePathData::Script { .. } => unreachable!(),
        }
    }

    fn determine_adversarial_path(
        self,
        db: &::salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        Some(
            determine_expect_file_path_without_extension(
                db,
                self,
                package_adversarials_dir,
                config,
            )
            .with_extension(adversarial_kind.as_str())
            .with_extension(config.adversarial_extension()),
        )
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        Some(self)
    }
}

pub fn determine_expect_file_path_without_extension(
    db: &::salsa::Db,
    module_path: ModulePath,
    root_dir: &Path,
    config: &VfsTestConfig,
) -> PathBuf {
    let path_parent_dir = determine_expect_file_path_parent_dir(db, module_path, root_dir, config);
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => path_parent_dir.join(format!(
            "{}.{}",
            match crate_path.kind(db) {
                CrateKind::Lib => "lib",
                CrateKind::Main => "main",
                CrateKind::Bin(_) => todo!(),
                CrateKind::IntegratedTest(_) => todo!(),
                CrateKind::Example => todo!(),
                CrateKind::Task => todo!(),
                CrateKind::Requirements => todo!(),
            },
            config.expect_file_extension().str()
        )),
        ModulePathData::Child { .. } => {
            path_parent_dir.with_extension(config.expect_file_extension().str())
        }
        ModulePathData::Script { .. } => unreachable!(),
    }
}

fn determine_expect_file_path_parent_dir(
    db: &::salsa::Db,
    module_path: ModulePath,
    root_dir: &Path,
    config: &VfsTestConfig,
) -> PathBuf {
    match module_path.data(db) {
        ModulePathData::Root(_) => root_dir.join(config.test_name()),
        ModulePathData::Child { parent, ident } => {
            determine_expect_file_path_parent_dir(db, parent, root_dir, config).join(ident.data(db))
        }
        ModulePathData::Script { .. } => unreachable!(),
    }
}
