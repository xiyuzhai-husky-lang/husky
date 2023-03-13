pub use husky_vfs::*;

use crate::*;

pub trait TokenTestUtils {
    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn token_plain_test<U>(&mut self, f: impl Fn(&Self, U))
    where
        U: VfsTestUnit;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn token_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        name: &str,
        f: impl Fn(&'a Self, U) -> R,
    ) where
        U: VfsTestUnit,
        R: salsa::DebugWithDb<Self>;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn token_expect_test_debug<'a, U, R>(&'a mut self, name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: std::fmt::Debug;
}

impl<Db> TokenTestUtils for Db
where
    Db: TokenDb + ?Sized,
{
    fn token_plain_test<U>(&mut self, f: impl Fn(&Self, U))
    where
        U: VfsTestUnit,
    {
        // todo: robustness
        self.vfs_plain_test(f);
    }

    fn token_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        name: &str,
        f: impl Fn(&'a Self, U) -> R,
    ) where
        U: VfsTestUnit,
        R: salsa::DebugWithDb<Self>,
    {
        // todo: robustness
        self.vfs_expect_test_debug_with_db(name, f)
    }

    fn token_expect_test_debug<'a, U, R>(&'a mut self, name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: std::fmt::Debug,
    {
        // todo: robustness
        self.vfs_expect_test_debug(name, &f)
    }
}
