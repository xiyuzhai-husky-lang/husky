pub use husky_token::test_utils::*;

use crate::*;

/// will have more robustness tests based on token level information than `TokenTestUtils`
pub trait AstTestUtils {
    /// only run to see whether the program will panic
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn ast_plain_test<U>(&mut self, task_name: &str, f: impl Fn(&Self, U))
    where
        U: VfsTestUnit;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn ast_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        task_name: &str,
        f: impl Fn(&'a Self, U) -> R,
    ) where
        U: VfsTestUnit,
        R: salsa::DebugWithDb<Self>;

    /// run to see whether the output agrees with previous
    /// it will invoke robustness test if environment variable `ROBUSTNESS_TEST` is set be a positive number
    fn ast_expect_test_debug<'a, U, R>(&'a mut self, task_name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: std::fmt::Debug;
}

impl<Db> AstTestUtils for Db
where
    Db: AstDb + ?Sized,
{
    fn ast_plain_test<U>(&mut self, task_name: &str, f: impl Fn(&Self, U))
    where
        U: VfsTestUnit,
    {
        // todo: robustness
        self.token_plain_test(task_name, f);
    }

    fn ast_expect_test_debug_with_db<'a, U, R>(
        &'a mut self,
        task_name: &str,
        f: impl Fn(&'a Self, U) -> R,
    ) where
        U: VfsTestUnit,
        R: salsa::DebugWithDb<Self>,
    {
        // todo: robustness
        self.token_expect_test_debug_with_db(task_name, f)
    }

    fn ast_expect_test_debug<'a, U, R>(&'a mut self, name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: std::fmt::Debug,
    {
        // todo: robustness
        self.token_expect_test_debug(name, f)
    }
}
