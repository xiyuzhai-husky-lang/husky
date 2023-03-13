use super::*;
pub trait VfsTestUnit: Sized {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self>;
    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf;
}

impl VfsTestUnit for PackagePath {
    fn collect_from_package_path(_db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        vec![package_path]
    }

    fn decide_expect_file_path(
        &self,
        _db: &dyn VfsDb,
        _task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf {
        package_expects_dir.with_extension(EXPECT_FILE_EXTENSION)
    }
}

impl VfsTestUnit for CratePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        db.collect_crates(package_path).unwrap_or_default()
    }

    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf {
        package_expects_dir.join(format!(
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
}

impl VfsTestUnit for ModulePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        db.collect_probable_modules(package_path)
    }

    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf {
        fn decide_expect_file_aux_path(
            db: &dyn VfsDb,
            module_path: ModulePath,
            task_name: &str,
            package_expects_dir: &Path,
        ) -> PathBuf {
            match module_path.data(db) {
                ModulePathData::Root(_) => package_expects_dir.join(task_name),
                ModulePathData::Child { parent, ident } => {
                    decide_expect_file_aux_path(db, parent, task_name, package_expects_dir)
                        .join(db.dt_ident(ident))
                }
            }
        }
        let aux_path = decide_expect_file_aux_path(db, *self, task_name, package_expects_dir);
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
}
