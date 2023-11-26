use super::*;

pub trait VfsTestUnit: Copy {
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

impl VfsTestUnit for PackagePath {
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
            .with_extension(config.test_name)
            .with_extension(config.expect_file_extension())
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

impl VfsTestUnit for CratePath {
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
            config.test_name,
            match self.crate_kind(db) {
                CrateKind::Lib => format!("lib"),
                CrateKind::Main => format!("main"),
                CrateKind::Bin(_) => todo!(),
                CrateKind::IntegratedTest(_) => todo!(),
                CrateKind::Example => todo!(),
            },
            config.expect_file_extension()
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

impl VfsTestUnit for ModulePath {
    fn collect_from_package_path(db: &::salsa::Db, package_path: PackagePath) -> Vec<Self> {
        db.collect_probable_modules(package_path)
    }

    fn determine_expect_file_path(
        &self,
        db: &::salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        fn determine_expect_file_aux_path(
            db: &::salsa::Db,
            module_path: ModulePath,
            package_expect_files_dir: &Path,
            config: &VfsTestConfig,
        ) -> PathBuf {
            match module_path.data(db) {
                ModulePathData::Root(_) => package_expect_files_dir.join(config.test_name),
                ModulePathData::Child { parent, ident } => {
                    determine_expect_file_aux_path(db, parent, package_expect_files_dir, config)
                        .join(db.dt_ident(ident))
                }
            }
        }
        let aux_path = determine_expect_file_aux_path(db, *self, package_expect_files_dir, config);
        match self.data(db) {
            ModulePathData::Root(crate_path) => aux_path.join(format!(
                "{}.{}",
                match crate_path.crate_kind(db) {
                    CrateKind::Lib => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Bin(_) => todo!(),
                    CrateKind::IntegratedTest(_) => todo!(),
                    CrateKind::Example => todo!(),
                },
                config.expect_file_extension()
            )),
            ModulePathData::Child { .. } => aux_path.with_extension(config.expect_file_extension()),
        }
    }

    fn determine_adversarial_path(
        self,
        db: &::salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        fn determine_adversarial_aux_path(
            db: &::salsa::Db,
            adversarial_kind: AdversarialKind,
            module_path: ModulePath,
            package_adversarials_dir: &Path,
            config: &VfsTestConfig,
        ) -> PathBuf {
            match module_path.data(db) {
                ModulePathData::Root(_) => package_adversarials_dir.join(config.test_name),
                ModulePathData::Child { parent, ident } => determine_adversarial_aux_path(
                    db,
                    adversarial_kind,
                    parent,
                    package_adversarials_dir,
                    config,
                )
                .join(db.dt_ident(ident)),
            }
        }
        let aux_path = determine_adversarial_aux_path(
            db,
            adversarial_kind,
            self,
            package_adversarials_dir,
            config,
        );
        Some(match self.data(db) {
            ModulePathData::Root(crate_path) => aux_path.join(format!(
                "{}.{adversarial_kind}.{ADVERSARIAL_EXTENSION}",
                match crate_path.crate_kind(db) {
                    CrateKind::Lib => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Bin(_) => todo!(),
                    CrateKind::IntegratedTest(_) => todo!(),
                    CrateKind::Example => todo!(),
                }
            )),
            ModulePathData::Child { .. } => aux_path
                .with_extension(adversarial_kind.as_str())
                .with_extension(ADVERSARIAL_EXTENSION),
        })
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        Some(self)
    }
}
