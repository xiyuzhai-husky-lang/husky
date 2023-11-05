pub use husky_token::test_utils::*;

use crate::*;

/// will have more robustness tests based on token level information than `TokenTestUtils`
pub trait AstTestUtils {
    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn ast_plain_test<U>(&mut self, f: impl Fn(&Self, U), config: &AstTestConfig)
    where
        U: VfsTestUnit + salsa::DebugWithDb<Self>;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn ast_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &AstTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: salsa::DebugWithDb<Self>;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn ast_expect_test_debug<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &AstTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: std::fmt::Debug;

    fn ast_expect_test_display<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &AstTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: std::fmt::Display;
}

impl<Db> AstTestUtils for Db
where
    Db: AstDb + ?Sized,
{
    fn ast_plain_test<U>(&mut self, f: impl Fn(&Self, U), config: &AstTestConfig)
    where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
    {
        // todo: robustness
        self.token_plain_test(f, config);
    }

    fn ast_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &AstTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: salsa::DebugWithDb<Self>,
    {
        // todo: robustness
        self.token_expect_test_debug_with_db(f, &config.token)
    }

    fn ast_expect_test_debug<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &AstTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: std::fmt::Debug,
    {
        // todo: robustness
        self.token_expect_test_debug(f, &config.token)
    }

    fn ast_expect_test_display<'a, U, R>(
        &'a mut self,
        f: impl Fn(&'a Self, U) -> R,
        config: &AstTestConfig<'a>,
    ) where
        U: VfsTestUnit + salsa::DebugWithDb<Self>,
        R: std::fmt::Display,
    {
        // todo: robustness
        self.token_expect_test_display(f, &config.token)
    }
}

pub struct AstTestConfig<'a> {
    token: TokenTestConfig<'a>,
}

impl<'a> AstTestConfig<'a> {
    pub fn new(test_name: &'a str) -> Self {
        Self {
            token: TokenTestConfig::new(test_name),
        }
    }
}

impl<'a> std::ops::Deref for AstTestConfig<'a> {
    type Target = TokenTestConfig<'a>;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl<'a> AstTestConfig<'a> {
    pub fn with_vfs_test_domains_config(
        mut self,
        test_domains_config: VfsTestDomainsConfig,
    ) -> Self {
        self.token = self.token.with_vfs_test_domains_config(test_domains_config);
        self
    }
}
