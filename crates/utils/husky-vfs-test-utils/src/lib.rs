#![feature(trait_upcasting)]
mod inner;

use husky_absolute_path::AbsolutePath;
use husky_entity_path::*;
use husky_package_path::{CrateKind, PackagePathData};
use husky_path_utils::*;
use husky_print_utils::p;
use husky_vfs::*;
use std::path::PathBuf;

pub trait VfsTestSupport: VfsDb {
    fn run_module_expect_tests<'a, R>(name: &str, f: impl Fn(&Self, EntityPath) -> &R)
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
    fn resolve_module_dir(&self, module: EntityPath) -> PathBuf {
        match self.db.dt_entity_path(module) {
            EntityPathData::CrateRoot(_) => self.package_expects_dir.join(self.name),
            EntityPathData::Childpath { parent, ident } => self
                .resolve_module_dir(parent)
                .join(self.db.dt_ident(ident)),
        }
    }

    fn resolve_path(&self, module: EntityPath) -> PathBuf {
        let dir = self.resolve_module_dir(module);
        match self.db.dt_entity_path(module) {
            EntityPathData::CrateRoot(crate_path) => dir.join(format!(
                "{}.txt",
                match crate_path.crate_kind(self.db) {
                    CrateKind::Library => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Binary(_) => todo!(),
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
    fn run_module_expect_tests<'a, R>(name: &str, f: impl Fn(&Self, EntityPath) -> &R)
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
            t(&db, name, base, out, &f);
        }
    }
}

fn t<T, R>(db: &T, name: &str, base: &Path, out: PathBuf, f: &impl Fn(&T, EntityPath) -> &R)
where
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
                let path = resolver.resolve_path(module);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_debug_eq(&f(&db, module))
            }
        });
}
