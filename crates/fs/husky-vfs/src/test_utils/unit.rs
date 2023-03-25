use super::*;

pub trait VfsTestUnit: Copy {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self>;
    fn determine_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expect_files_dir: &Path,
    ) -> PathBuf;
    fn determine_adversarial_path(
        self,
        db: &dyn VfsDb,
        adversarial_kind: AdversarialKind,
        task_name: &str,
        package_adversarials_dir: &Path,
    ) -> Option<PathBuf>;
    fn module(self) -> Option<ModulePath>;
}

impl VfsTestUnit for PackagePath {
    fn collect_from_package_path(_db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        vec![package_path]
    }

    fn determine_expect_file_path(
        &self,
        _db: &dyn VfsDb,
        task_name: &str,
        package_expect_files_dir: &Path,
    ) -> PathBuf {
        package_expect_files_dir
            .with_extension(task_name)
            .with_extension(EXPECT_FILE_EXTENSION)
    }

    fn determine_adversarial_path(
        self,
        _db: &dyn VfsDb,
        _adversarial_kind: AdversarialKind,
        _task_name: &str,
        _package_adversarials_dir: &Path,
    ) -> Option<PathBuf> {
        None
    }

    fn module(self) -> Option<ModulePath> {
        None
    }
}

impl VfsTestUnit for CratePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        db.collect_crates(package_path).unwrap_or_default()
    }

    fn determine_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expect_files_dir: &Path,
    ) -> PathBuf {
        package_expect_files_dir.join(format!(
            "{}/{}.{EXPECT_FILE_EXTENSION}",
            task_name,
            match self.crate_kind(db) {
                CrateKind::Library => format!("lib"),
                CrateKind::Main => format!("main"),
                CrateKind::Binary(_) => todo!(),
                CrateKind::StandaloneTest(_) => todo!(),
            }
        ))
    }

    fn determine_adversarial_path(
        self,
        _db: &dyn VfsDb,
        _adversarial_kind: AdversarialKind,
        _task_name: &str,
        _package_adversarials_dir: &Path,
    ) -> Option<PathBuf> {
        None
    }

    fn module(self) -> Option<ModulePath> {
        None
    }
}

impl VfsTestUnit for ModulePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        db.collect_probable_modules(package_path)
    }

    fn determine_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expect_files_dir: &Path,
    ) -> PathBuf {
        fn determine_expect_file_aux_path(
            db: &dyn VfsDb,
            module_path: ModulePath,
            task_name: &str,
            package_expect_files_dir: &Path,
        ) -> PathBuf {
            match module_path.data(db) {
                ModulePathData::Root(_) => package_expect_files_dir.join(task_name),
                ModulePathData::Child { parent, ident } => {
                    determine_expect_file_aux_path(db, parent, task_name, package_expect_files_dir)
                        .join(db.dt_ident(ident))
                }
            }
        }
        let aux_path =
            determine_expect_file_aux_path(db, *self, task_name, package_expect_files_dir);
        match self.data(db) {
            ModulePathData::Root(crate_path) => aux_path.join(format!(
                "{}.{EXPECT_FILE_EXTENSION}",
                match crate_path.crate_kind(db) {
                    CrateKind::Library => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Binary(_) => todo!(),
                    CrateKind::StandaloneTest(_) => todo!(),
                }
            )),
            ModulePathData::Child { .. } => aux_path.with_extension(EXPECT_FILE_EXTENSION),
        }
    }

    fn determine_adversarial_path(
        self,
        db: &dyn VfsDb,
        adversarial_kind: AdversarialKind,
        task_name: &str,
        package_adversarials_dir: &Path,
    ) -> Option<PathBuf> {
        fn determine_adversarial_aux_path(
            db: &dyn VfsDb,
            adversarial_kind: AdversarialKind,
            module_path: ModulePath,
            task_name: &str,
            package_adversarials_dir: &Path,
        ) -> PathBuf {
            match module_path.data(db) {
                ModulePathData::Root(_) => package_adversarials_dir.join(task_name),
                ModulePathData::Child { parent, ident } => determine_adversarial_aux_path(
                    db,
                    adversarial_kind,
                    parent,
                    task_name,
                    package_adversarials_dir,
                )
                .join(db.dt_ident(ident)),
            }
        }
        let aux_path = determine_adversarial_aux_path(
            db,
            adversarial_kind,
            self,
            task_name,
            package_adversarials_dir,
        );
        Some(match self.data(db) {
            ModulePathData::Root(crate_path) => aux_path.join(format!(
                "{}.{adversarial_kind}.{ADVERSARIAL_EXTENSION}",
                match crate_path.crate_kind(db) {
                    CrateKind::Library => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Binary(_) => todo!(),
                    CrateKind::StandaloneTest(_) => todo!(),
                }
            )),
            ModulePathData::Child { .. } => aux_path
                .with_extension(adversarial_kind.as_str())
                .with_extension(ADVERSARIAL_EXTENSION),
        })
    }

    fn module(self) -> Option<ModulePath> {
        Some(self)
    }
}
