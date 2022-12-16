mod inner;

use husky_absolute_path::AbsolutePath;
use husky_entity_path::*;
use husky_package_path::PackagePathData;
use husky_path_utils::*;
use husky_print_utils::p;
use husky_vfs::*;
use std::path::PathBuf;

pub trait VfsTestSupport: VfsDb {
    fn mimic_dir(&self, module: EntityPath) -> PathBuf;
    fn mimic_path(&self, module: EntityPath) -> PathBuf;
    fn run_module_expect_tests<'a, R>(f: impl Fn(&Self, EntityPath) -> &R)
    where
        Self: Default + 'a,
        R: std::fmt::Debug;
}

impl<T> VfsTestSupport for T
where
    T: VfsDb,
{
    fn mimic_dir(&self, module: EntityPath) -> PathBuf {
        match self.dt_entity_path(module) {
            EntityPathData::Crate { package, kind } => {
                self.package_dir(package).as_ref().unwrap().join("ast")
            }
            EntityPathData::Childpath { parent, ident } => {
                self.mimic_dir(parent).join(self.dt_ident(ident))
            }
        }
    }

    fn mimic_path(&self, module: EntityPath) -> PathBuf {
        let dir = self.mimic_dir(module);
        dir.join(format!(
            "{}.ast.txt",
            match self.dt_entity_path(module) {
                EntityPathData::Crate { package, kind } => match kind {
                    CratePathKind::Library => "lib",
                    CratePathKind::Main => "main",
                    CratePathKind::Binary(_) => todo!(),
                },
                EntityPathData::Childpath { ident, .. } => self.dt_ident(ident),
            }
        ))
    }

    fn run_module_expect_tests<'a, R>(f: impl Fn(&Self, EntityPath) -> &R)
    where
        Self: Default + 'a,
        R: std::fmt::Debug,
    {
        let db = Self::default();
        let cargo_manifest_dir = cargo_manifest_dir().unwrap();
        let examples_dir = cargo_manifest_dir.join("tests/examples");
        std::fs::create_dir_all(&examples_dir);
        collect_package_dirs(examples_dir)
            .into_iter()
            .for_each(|path| {
                let package =
                    db.it_package_path(PackagePathData::Local(AbsolutePath::new(&path).unwrap()));
                for module in db.all_possible_modules(package).unwrap() {
                    let path = db.mimic_path(module);
                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                    p!(path);
                    expect_test::expect_file![path].assert_debug_eq(&f(&db, module))
                }
            });
    }
}
