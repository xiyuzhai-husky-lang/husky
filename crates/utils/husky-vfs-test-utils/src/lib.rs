#![feature(trait_upcasting)]
mod inner;
mod macros;

use husky_absolute_path::AbsolutePath;
use husky_entity_path::*;
use husky_package_path::{CrateKind, CratePath, PackagePathData};
use husky_path_utils::*;
use husky_print_utils::p;
use husky_vfs::*;
use std::path::PathBuf;

pub trait VfsTestSupport: VfsDb {
    fn expect_test_crates<'a, R>(name: &str, f: impl Fn(&Self, CratePath) -> &R)
    where
        Self: Default + 'a,
        R: std::fmt::Debug;

    fn expect_test_modules<'a, R>(name: &str, f: impl Fn(&Self, EntityPath) -> &R)
    where
        Self: Default + 'a,
        R: std::fmt::Debug;
}

struct TestPathResolver<'a> {
    db: &'a dyn VfsDb,
    name: &'a str,
    package_expects_dir: PathBuf,
}

impl<'a> TestPathResolver<'a> {
    // return the folder containing submodules
    fn decide_module_dir(&self, module: EntityPath) -> PathBuf {
        match self.db.dt_entity_path(module) {
            EntityPathData::CrateRoot(_) => self.package_expects_dir.join(self.name),
            EntityPathData::Childpath { parent, ident } => {
                self.decide_module_dir(parent).join(self.db.dt_ident(ident))
            }
        }
    }

    fn decide_crate_expect_file_path(&self, crate_path: CratePath) -> PathBuf {
        self.package_expects_dir.join(format!(
            "{}/{}",
            self.name,
            match crate_path.crate_kind(self.db) {
                CrateKind::Library => "lib.txt",
                CrateKind::Main => "main.txt",
                CrateKind::Binary(_) => todo!(),
                CrateKind::StandaloneTest(_) => todo!(),
            }
        ))
    }

    fn decide_module_expect_file_path(&self, module: EntityPath) -> PathBuf {
        let dir = self.decide_module_dir(module);
        match self.db.dt_entity_path(module) {
            EntityPathData::CrateRoot(crate_path) => dir.join(format!(
                "{}.txt",
                match crate_path.crate_kind(self.db) {
                    CrateKind::Library => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Binary(_) => todo!(),
                    CrateKind::StandaloneTest(_) => todo!(),
                }
            )),
            EntityPathData::Childpath { ident, .. } => dir.with_extension("txt"),
        }
    }
}

impl<T> VfsTestSupport for T
where
    T: VfsDb,
{
    fn expect_test_crates<'a, R>(name: &str, f: impl Fn(&Self, CratePath) -> &R)
    where
        Self: Default + 'a,
        R: std::fmt::Debug,
    {
        let db = Self::default();
        let cargo_manifest_dir = cargo_manifest_dir().unwrap();
        for (base, out) in [
            (
                db.vfs_config().library_dir(),
                cargo_manifest_dir.join("expect-files/library"),
            ),
            (
                db.vfs_config().examples_dir(),
                cargo_manifest_dir.join("expect-files/examples"),
            ),
        ] {
            expect_test_crates(&db, name, base, out, &f);
        }
    }

    fn expect_test_modules<'a, R>(name: &str, f: impl Fn(&Self, EntityPath) -> &R)
    where
        Self: Default + 'a,
        R: std::fmt::Debug,
    {
        let db = Self::default();
        let cargo_manifest_dir = cargo_manifest_dir().unwrap();
        for (base, out) in [
            (
                db.vfs_config().library_dir(),
                cargo_manifest_dir.join("expect-files/library"),
            ),
            (
                db.vfs_config().examples_dir(),
                cargo_manifest_dir.join("expect-files/examples"),
            ),
        ] {
            expect_test_modules(&db, name, base, out, &f);
        }
    }
}

fn expect_test_crates<T, R>(
    db: &T,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&T, CratePath) -> &R,
) where
    T: VfsDb,
    R: std::fmt::Debug,
{
    std::fs::create_dir_all(&out);
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package_path = db.it_package_path(PackagePathData::Local(
                AbsolutePath::new(&path.to_logical_path(base)).unwrap(),
            ));
            let resolver = TestPathResolver {
                db,
                name,
                package_expects_dir: path.to_logical_path(&out),
            };
            use salsa::DebugWithDb;
            for crate_path in db.collect_crates(package_path).unwrap() {
                let path = resolver.decide_crate_expect_file_path(crate_path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_debug_eq(&f(&db, crate_path));
            }
        });
}

fn expect_test_modules<T, R>(
    db: &T,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&T, EntityPath) -> &R,
) where
    T: VfsDb,
    R: std::fmt::Debug,
{
    std::fs::create_dir_all(&out);
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package = db.it_package_path(PackagePathData::Local(
                AbsolutePath::new(&path.to_logical_path(base)).unwrap(),
            ));
            let resolver = TestPathResolver {
                db,
                name,
                package_expects_dir: path.to_logical_path(&out),
            };
            for module in db.collect_possible_modules(package).unwrap() {
                use salsa::DebugWithDb;
                let path = resolver.decide_module_expect_file_path(module);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_debug_eq(&f(&db, module))
            }
        });
}
