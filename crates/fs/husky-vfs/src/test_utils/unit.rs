use super::*;
use crate::jar::VfsDb;

pub trait IsVfsTestUnit: Copy {
    fn collect_from_package_path(db: &::salsa::Db, package_path: PackagePath) -> Vec<Self>;
    fn determine_expect_file_path(
        self,
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
        self,
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
        package_path
            .crate_paths(db)
            .expect("no vfs error in testing")
            .to_vec()
    }

    fn determine_expect_file_path(
        self,
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
                CrateKind::Task => format!("task"),
                CrateKind::Requirements => format!("requirements"),
                CrateKind::Bin(_) => todo!(),
                CrateKind::IntegratedTest(_) => todo!(),
                CrateKind::Example => todo!(),
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
        self,
        db: &::salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        self.relative_stem(db)
            .to_logical_path(package_expect_files_dir.join(config.test_name()))
            .with_extension(config.expect_file_extension().str())
    }

    fn determine_adversarial_path(
        self,
        db: &::salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        Some(
            self.relative_stem(db)
                .to_logical_path(package_adversarials_dir.join(config.test_name()))
                .with_extension(adversarial_kind.as_str())
                .with_extension(config.adversarial_extension()),
        )
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        Some(self)
    }
}
