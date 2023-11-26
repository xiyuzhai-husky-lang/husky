pub use husky_vfs::{VfsTestConfig, VfsTestDomainsConfig, VfsTestUnit, VfsTestUtils};
use salsa::{test_utils::Db, Database};

use crate::*;

/// will have more robustness tests based on token level information than `VfsTestUtils`
pub trait TokenTestUtils: VfsTestUtils {
    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn token_plain_test<U>(&mut self, f: impl Fn(&Db, U), config: &VfsTestConfig)
    where
        U: VfsTestUnit;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn token_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Db, U) -> R,
        config: &TokenTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: salsa::DebugWithDb;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn token_expect_test_debug<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Db, U) -> R,
        config: &TokenTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Debug;

    fn token_expect_test_display<U, R>(
        &mut self,
        f: impl Fn(&Db, U) -> R,
        config: &TokenTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Display;
}

impl TokenTestUtils for Db
where
    Db: VfsTestUtils,
{
    fn token_plain_test<U>(&mut self, f: impl Fn(&Db, U), config: &VfsTestConfig)
    where
        U: VfsTestUnit,
    {
        // todo: robustness
        self.vfs_plain_test(f, config);
    }

    fn token_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Db, U) -> R,
        config: &TokenTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: salsa::DebugWithDb,
    {
        // todo: robustness
        self.vfs_expect_test_debug_with_db(f, &config.vfs)
    }

    fn token_expect_test_debug<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Db, U) -> R,
        config: &TokenTestConfig,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Debug,
    {
        // todo: robustness
        self.vfs_expect_test_debug(&f, &config.vfs)
    }

    fn token_expect_test_display<U, R>(&mut self, f: impl Fn(&Db, U) -> R, config: &TokenTestConfig)
    where
        U: VfsTestUnit + salsa::DebugWithDb,
        R: std::fmt::Display,
    {
        // todo: robustness
        self.vfs_expect_test_display(&f, &config.vfs)
    }
}

pub struct TokenTestConfig<'a> {
    vfs: VfsTestConfig<'a>,
}

impl<'a> std::ops::Deref for TokenTestConfig<'a> {
    type Target = VfsTestConfig<'a>;

    fn deref(&self) -> &Self::Target {
        &self.vfs
    }
}

impl<'a> TokenTestConfig<'a> {
    pub fn new(test_name: &'a str) -> Self {
        Self {
            vfs: VfsTestConfig::new(test_name),
        }
    }

    pub fn with_vfs_test_domains_config(
        mut self,
        test_domains_config: VfsTestDomainsConfig,
    ) -> Self {
        self.vfs = self.vfs.with_vfs_test_domains_config(test_domains_config);
        self
    }

    pub fn with_expect_file_extension(mut self, extension: &'a str) -> Self {
        self.vfs = self.vfs.with_expect_file_extension(extension);
        self
    }
}
