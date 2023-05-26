mod ad_hoc_package;
mod adversarial_test;
mod domain;
mod expect_test;
mod unit;

pub use self::adversarial_test::*;
pub use self::expect_test::*;
pub use self::unit::*;

use self::domain::*;
use crate::*;
use husky_path_utils::*;
use salsa::DebugWithDb;

use std::path::PathBuf;

pub trait VfsTestUtils: VfsDb {
    // toolchain
    fn dev_toolchain(&self) -> ToolchainResult<Toolchain> {
        let library_path = derive_library_path_from_cargo_manifest_dir()?;
        let db = <Self as salsa::DbWithJar<VfsJar>>::as_jar_db(&self);
        Ok(Toolchain::new(
            db,
            ToolchainData::Local {
                library_path: DiffPath::try_new(db, &library_path).unwrap(),
            },
        ))
    }

    fn dev_path_menu(&self) -> ToolchainResult<&VfsPathMenu> {
        let toolchain = self.dev_toolchain()?;
        Ok(self.vfs_path_menu(toolchain))
    }

    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_plain_test<U>(&mut self, task_name: &str, f: impl Fn(&Self, U))
    where
        U: VfsTestUnit,
    {
        for _dir in test_dirs() {
            let toolchain = self.dev_toolchain().unwrap();
            for domain in vfs_test_suites() {
                for (path, name) in collect_package_relative_dirs(
                    <Self as salsa::DbWithJar<WordJar>>::as_jar_db(self),
                    &domain.src_base(),
                )
                .into_iter()
                {
                    let vfs_db = <Self as salsa::DbWithJar<VfsJar>>::as_jar_db(self);
                    let package_path = PackagePath::new_local_package(
                        vfs_db,
                        toolchain,
                        name,
                        &path.to_logical_path(&domain.src_base()),
                    )
                    .unwrap();
                    for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path)
                    {
                        f(self, unit);
                        if let Some(adversarials_base) = domain.adversarials_base() {
                            vfs_adversarial_test(
                                self,
                                task_name,
                                &path.to_logical_path(adversarials_base),
                                unit,
                                &f,
                            )
                        }
                    }
                }
            }
        }
    }

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        name: &str,
        f: impl Fn(&'a Self, U) -> R,
    ) where
        U: VfsTestUnit,
        R: salsa::DebugWithDb<Self>,
    {
        vfs_expect_test(self, name, |db, u| {
            format!("{:#?}", &f(unsafe { std::mem::transmute(db) }, u).debug(db))
        })
    }

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_expect_test_debug<'a, U, R>(&'a mut self, name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: std::fmt::Debug,
    {
        vfs_expect_test(self, name, |db, u| {
            format!("{:#?}", &f(unsafe { std::mem::transmute(db) }, u))
        })
    }
}

const EXPECT_FILE_EXTENSION: &'static str = "md";
const ADVERSARIAL_EXTENSION: &'static str = "json";

impl<Db> VfsTestUtils for Db where Db: VfsDb + ?Sized {}

fn test_dirs() -> Vec<PathBuf> {
    let env = HuskyDevPathEnv::new();
    vec![
        env.lang_dev_library_dir().to_owned(),
        env.lang_dev_examples_dir().to_owned(),
    ]
}
