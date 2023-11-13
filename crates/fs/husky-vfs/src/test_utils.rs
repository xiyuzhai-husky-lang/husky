mod ad_hoc_package;
mod adversarial_test;
mod domain;
mod expect_test;
mod unit;

pub use self::adversarial_test::*;
pub use self::domain::*;
pub use self::unit::*;

use self::expect_test::*;
use crate::*;
use husky_path_utils::*;
use std::path::PathBuf;

pub trait VfsTestUtils: VfsDb {
    // toolchain
    fn dev_toolchain(&self) -> ToolchainResult<Toolchain> {
        let library_path = find_lang_dev_library_path()?;
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
    fn vfs_plain_test<U>(&mut self, f: impl Fn(&Self, U), config: &VfsTestConfig)
    where
        U: VfsTestUnit,
    {
        let toolchain = self.dev_toolchain().unwrap();
        for test_suite in config.test_domains() {
            for (path, name) in collect_package_relative_dirs(
                <Self as salsa::DbWithJar<CowordJar>>::as_jar_db(self),
                &test_suite.src_base(),
            )
            .into_iter()
            {
                let vfs_db = <Self as salsa::DbWithJar<VfsJar>>::as_jar_db(self);
                let package_path = PackagePath::new_local_or_toolchain_package(
                    vfs_db,
                    toolchain,
                    name,
                    &path.to_logical_path(&test_suite.src_base()),
                )
                .unwrap();
                for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path) {
                    f(self, unit);
                    if let Some(adversarials_base) = test_suite.adversarials_base() {
                        vfs_adversarial_test(
                            self,
                            &path.to_logical_path(adversarials_base),
                            unit,
                            &f,
                            config,
                        )
                    }
                }
            }
        }
    }

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: salsa::DebugWithDb<Self>,
    {
        vfs_expect_test(
            self,
            |db, u| format!("{:#?}", &f(unsafe { std::mem::transmute(db) }, u).debug(db)),
            config,
        )
    }

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_expect_test_debug<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: std::fmt::Debug,
    {
        vfs_expect_test(
            self,
            |db, u| format!("{:#?}", &f(unsafe { std::mem::transmute(db) }, u)),
            config,
        )
    }

    fn vfs_expect_test_display<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: std::fmt::Display,
    {
        self::expect_test::vfs_expect_test(
            self,
            |db, u| format!("{}", &f(unsafe { std::mem::transmute(db) }, u)),
            config,
        )
    }
}

const ADVERSARIAL_EXTENSION: &'static str = "json";

impl<Db> VfsTestUtils for Db where Db: VfsDb + ?Sized {}

pub struct VfsTestConfig<'a> {
    test_name: &'a str,
    expect_file_extension: &'a str,
    test_domains_config: VfsTestDomainsConfig,
}

impl<'a> VfsTestConfig<'a> {
    pub fn new(test_name: &'a str) -> Self {
        Self {
            test_name,
            expect_file_extension: "md",
            test_domains_config: Default::default(),
        }
    }

    pub fn with_expect_file_extension(mut self, expect_file_extension: &'a str) -> Self {
        self.expect_file_extension = expect_file_extension;
        self
    }

    pub fn expect_file_extension(&self) -> &str {
        self.expect_file_extension
    }
}
