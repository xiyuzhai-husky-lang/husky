use crate::*;
use husky_path_utils::*;
use salsa::DebugWithDb;
use std::path::PathBuf;

pub trait VfsTestSupport: VfsDb {
    fn test_crates(f: impl Fn(&Self, CratePath))
    where
        Self: Default;

    fn test_probable_modules(f: impl Fn(&Self, ModulePath))
    where
        Self: Default;

    fn expect_test_packages_debug_result<T, E>(
        name: &str,
        f: impl Fn(&Self, PackagePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug;

    fn expect_test_crates_debug_result_with_db<T, E>(
        name: &str,
        f: impl Fn(&Self, CratePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: salsa::DebugWithDb<Self> + ?Sized,
        E: salsa::DebugWithDb<Self>;

    fn expect_test_crates_debug_result<T, E>(
        name: &str,
        f: impl Fn(&Self, CratePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug;

    fn expect_test_probable_modules_debug_with_db<R>(
        name: &str,
        f: impl Fn(&Self, ModulePath) -> &R,
    ) where
        Self: Default,
        R: salsa::DebugWithDb<Self> + ?Sized;

    fn expect_test_probable_modules_debug_result_with_db<T, E>(
        name: &str,
        f: impl Fn(&Self, ModulePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: salsa::DebugWithDb<Self> + ?Sized,
        E: salsa::DebugWithDb<Self>;

    fn expect_test_probable_modules_debug<T, E>(
        name: &str,
        f: impl Fn(&Self, ModulePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug;
}

struct TestPathResolver<'a> {
    db: &'a dyn VfsDb,
    name: &'a str,
    package_expects_dir: PathBuf,
}

impl<'a> TestPathResolver<'a> {
    // return the folder containing submodules
    fn decide_module_dir(&self, module: ModulePath) -> PathBuf {
        match module.data(self.db) {
            ModulePathData::Root(_) => self.package_expects_dir.join(self.name),
            ModulePathData::Child { parent, ident } => {
                self.decide_module_dir(parent).join(self.db.dt_ident(ident))
            }
        }
    }

    fn decide_crate_expect_file_path(&self, crate_path: CratePath) -> PathBuf {
        self.package_expects_dir.join(format!(
            "{}/{}.{EXPECT_FILE_EXTENSION}",
            self.name,
            match crate_path.crate_kind(self.db) {
                CrateKind::Library => format!("lib"),
                CrateKind::Main => format!("main"),
                CrateKind::Binary(_) => todo!(),
                CrateKind::StandaloneTest(_) => todo!(),
            }
        ))
    }

    fn decide_module_expect_file_path(&self, module: ModulePath) -> PathBuf {
        let dir = self.decide_module_dir(module);
        match module.data(self.db) {
            ModulePathData::Root(crate_path) => dir.join(format!(
                "{}.{EXPECT_FILE_EXTENSION}",
                match crate_path.crate_kind(self.db) {
                    CrateKind::Library => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Binary(_) => todo!(),
                    CrateKind::StandaloneTest(_) => todo!(),
                }
            )),
            ModulePathData::Child { .. } => dir.with_extension(EXPECT_FILE_EXTENSION),
        }
    }
}

const EXPECT_FILE_EXTENSION: &'static str = "md";

impl<Db> VfsTestSupport for Db
where
    Db: VfsDb,
{
    fn test_crates(f: impl Fn(&Self, CratePath))
    where
        Self: Default,
    {
        let db = Self::default();
        for dir in test_dirs() {
            test_crates(&db, &dir, &f);
        }
    }

    fn test_probable_modules(f: impl Fn(&Self, ModulePath))
    where
        Self: Default,
    {
        let db = Self::default();
        for dir in test_dirs() {
            test_probable_modules(&db, &dir, &f);
        }
    }

    fn expect_test_crates_debug_result_with_db<T, E>(
        name: &str,
        f: impl Fn(&Self, CratePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: salsa::DebugWithDb<Self> + ?Sized,
        E: salsa::DebugWithDb<Self>,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_crates(&db, name, &base, out, &f, |db, r| {
                format!("{:#?}", &r.debug(db))
            });
        }
    }

    fn expect_test_packages_debug_result<T, E>(
        name: &str,
        f: impl Fn(&Self, PackagePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_packages(&db, name, &base, out, &f, |_db, r| format!("{:#?}", r));
        }
    }

    fn expect_test_crates_debug_result<T, E>(
        name: &str,
        f: impl Fn(&Self, CratePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_crates(&db, name, &base, out, &f, |_db, r| format!("{:#?}", r));
        }
    }

    fn expect_test_probable_modules_debug_with_db<R>(
        name: &str,
        f: impl for<'a> Fn(&'a Self, ModulePath) -> &'a R,
    ) where
        Self: Default,
        R: salsa::DebugWithDb<Self> + ?Sized,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_probable_modules_debug_with_db(&db, name, &base, out, &f, |db, r| {
                format!("{:#?}", (&r).debug(db))
            });
        }
    }

    fn expect_test_probable_modules_debug_result_with_db<T, E>(
        name: &str,
        f: impl Fn(&Self, ModulePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: salsa::DebugWithDb<Self> + ?Sized,
        E: salsa::DebugWithDb<Self>,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_probable_modules_debug_result_with_db(
                &db,
                name,
                &base,
                out,
                &f,
                |db, r| format!("{:#?}", r.debug(db)),
            );
        }
    }

    fn expect_test_probable_modules_debug<T, E>(
        name: &str,
        f: impl Fn(&Self, ModulePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_probable_modules_debug_result_with_db(
                &db,
                name,
                &base,
                out,
                &f,
                |_db, r| format!("{:#?}", r),
            );
        }
    }
}

fn test_dirs() -> Vec<PathBuf> {
    let env = HuskyDevPathEnv::new();
    vec![
        env.lang_dev_library_dir().to_owned(),
        env.lang_dev_examples_dir().to_owned(),
    ]
}

fn expect_test_base_outs() -> Vec<(PathBuf, PathBuf)> {
    let env = HuskyDevPathEnv::new();
    vec![
        (
            env.lang_dev_library_dir().to_owned(),
            env.cargo_manifest_dir().join("expect-files/library"),
        ),
        (
            env.lang_dev_examples_dir().to_owned(),
            env.cargo_manifest_dir().join("expect-files/examples"),
        ),
    ]
}

fn test_crates<T>(db: &T, dir: &Path, f: &impl Fn(&T, CratePath))
where
    T: VfsDb,
{
    let toolchain = db.dev_toolchain();
    collect_husky_package_dirs(dir)
        .into_iter()
        .for_each(|path| {
            let package_path = PackagePath::new_local(db, toolchain, &path).unwrap();
            for crate_path in db.collect_crates(toolchain, package_path).unwrap() {
                f(db, crate_path)
            }
        });
}

fn test_probable_modules<T>(db: &T, dir: &Path, f: &impl Fn(&T, ModulePath))
where
    T: VfsDb,
{
    let toolchain = db.dev_toolchain();
    collect_husky_package_dirs(dir)
        .into_iter()
        .for_each(|path| {
            let package_path = PackagePath::new_local(db, toolchain, &path).unwrap();
            for entity_path in db.collect_probable_modules(package_path).unwrap() {
                f(db, entity_path)
            }
        });
}

fn expect_test_packages<Db, T, E>(
    db: &Db,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&Db, PackagePath) -> Result<&T, E>,
    p: impl Fn(&Db, Result<&T, E>) -> String,
) where
    Db: VfsDb,
    T: ?Sized,
{
    let toolchain = db.dev_toolchain();
    std::fs::create_dir_all(&out).unwrap();
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package_path =
                PackagePath::new_local(db, toolchain, &path.to_logical_path(base)).unwrap();
            let resolver = TestPathResolver {
                db,
                name,
                package_expects_dir: path.to_logical_path(&out),
            };
            let path = path.to_logical_path(&out);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            expect_test::expect_file![path].assert_eq(&p(&db, f(&db, package_path)));
        });
}

fn expect_test_crates<Db, T, E>(
    db: &Db,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&Db, CratePath) -> Result<&T, E>,
    p: impl Fn(&Db, Result<&T, E>) -> String,
) where
    Db: VfsDb,
    T: ?Sized,
{
    let toolchain = db.dev_toolchain();
    std::fs::create_dir_all(&out).unwrap();
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package_path =
                PackagePath::new_local(db, toolchain, &path.to_logical_path(base)).unwrap();
            let resolver = TestPathResolver {
                db,
                name,
                package_expects_dir: path.to_logical_path(&out),
            };

            for crate_path in db.collect_crates(toolchain, package_path).unwrap() {
                let path = resolver.decide_crate_expect_file_path(crate_path);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_eq(&p(&db, f(&db, crate_path)));
            }
        });
}

fn expect_test_probable_modules_debug_with_db<Db, R: ?Sized>(
    db: &Db,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&Db, ModulePath) -> &R,
    p: impl for<'a> Fn(&'a Db, &'a R) -> String,
) where
    Db: VfsDb,
{
    std::fs::create_dir_all(&out).expect("failed_to_create_dir_all");
    let toolchain = db.dev_toolchain();
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package =
                PackagePath::new_local(db, toolchain, &path.to_logical_path(base)).unwrap();
            let resolver = TestPathResolver {
                db,
                name,
                package_expects_dir: path.to_logical_path(&out),
            };
            for module in db.collect_probable_modules(package).unwrap() {
                let path = resolver.decide_module_expect_file_path(module);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_eq(&p(&db, f(&db, module)))
            }
        });
}

fn expect_test_probable_modules_debug_result_with_db<Db, T: ?Sized, E>(
    db: &Db,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&Db, ModulePath) -> Result<&T, E>,
    p: impl for<'a> Fn(&'a Db, Result<&'a T, E>) -> String,
) where
    Db: VfsDb,
{
    std::fs::create_dir_all(&out).expect("failed_to_create_dir_all");
    let toolchain = db.dev_toolchain();
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package =
                PackagePath::new_local(db, toolchain, &path.to_logical_path(base)).unwrap();
            let resolver = TestPathResolver {
                db,
                name,
                package_expects_dir: path.to_logical_path(&out),
            };
            for module in db.collect_probable_modules(package).unwrap() {
                let path = resolver.decide_module_expect_file_path(module);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_eq(&p(&db, f(&db, module)))
            }
        });
}
