mod adversarial_test;
pub mod db;
mod domain;
mod expect_test;
mod unit;

pub use self::adversarial_test::*;
pub use self::db::*;
pub use self::domain::*;
pub use self::unit::*;

use self::expect_test::*;
use crate::*;
use husky_path_utils::*;
use salsa::Db;
use std::path::PathBuf;

pub trait VfsTestUtils: Default + std::ops::Deref<Target = Db> + std::ops::DerefMut {
    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_plain_test<U>(&mut self, f: impl Fn(&::salsa::Db, U), config: &VfsTestConfig)
    where
        U: VfsTestUnit;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_expect_test_debug_with_db<'a, U, R>(
        &mut self,
        f: impl Fn(&'a ::salsa::Db, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: salsa::DebugWithDb;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_expect_test_debug<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a ::salsa::Db, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Debug;

    fn vfs_expect_test_display<U, R>(
        &mut self,
        f: impl Fn(&::salsa::Db, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Display;
}

const ADVERSARIAL_EXTENSION: &'static str = "json";

impl<DB> VfsTestUtils for DB
where
    DB: Default + std::ops::Deref<Target = Db> + std::ops::DerefMut,
{
    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn vfs_plain_test<U>(&mut self, f: impl Fn(&::salsa::Db, U), config: &VfsTestConfig)
    where
        U: VfsTestUnit,
    {
        let toolchain = self.dev_toolchain().unwrap();
        for test_suite in config.test_domains() {
            for (path, _name) in
                collect_package_relative_dirs(self, &test_suite.src_base()).into_iter()
            {
                let package_path = PackagePath::new_local_or_toolchain_package(
                    self,
                    toolchain,
                    &path.to_logical_path(&test_suite.src_base()),
                )
                .unwrap();
                for unit in <U as VfsTestUnit>::collect_from_package_path(self, package_path) {
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
        &mut self,
        f: impl Fn(&'a ::salsa::Db, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: salsa::DebugWithDb,
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
        f: impl Fn(&'a ::salsa::Db, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Debug,
    {
        vfs_expect_test(
            self,
            |db, u| format!("{:#?}", &f(unsafe { std::mem::transmute(db) }, u)),
            config,
        )
    }

    fn vfs_expect_test_display<U, R>(
        &mut self,
        f: impl Fn(&::salsa::Db, U) -> R,
        config: &VfsTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Display,
    {
        self::expect_test::vfs_expect_test(self, |db, u| format!("{}", &f(db, u)), config)
    }
}
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
