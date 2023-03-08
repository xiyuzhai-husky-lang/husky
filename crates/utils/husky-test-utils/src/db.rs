use crate::*;

pub trait AsTestDb {
    fn test_all<TestInput>(&self, f: impl Fn(&Self, &TestInput))
    where
        TestInput: AsTestInput<Self>;

    fn expect_test_all<TestInput, R>(&self, f: impl Fn(&Self, &TestInput) -> R)
    where
        R: std::fmt::Debug;

    fn expect_test_all_with_db<TestInput, R>(&self, f: impl Fn(&Self, &TestInput) -> R)
    where
        R: salsa::DebugWithDb<Self>;

    fn expect_test_all_results<TestInput, T, E>(
        &self,
        dir: &std::path::Path,
        f: impl for<'a> Fn(&'a Self, &'a TestInput) -> Result<&'a T, E>,
    ) where
        T: std::fmt::Debug,
        E: std::fmt::Debug;

    fn expect_test_all_results_with_db<TestInput, T, E>(
        &self,
        dir: &std::path::Path,
        f: impl for<'a> Fn(&'a Self, &'a TestInput) -> Result<&'a T, E>,
    ) where
        T: salsa::DebugWithDb<Self>,
        E: salsa::DebugWithDb<Self>;
}

impl<Db: ?Sized> AsTestDb for Db {
    fn test_all<TestInput>(&self, f: impl Fn(&Self, &TestInput))
    where
        TestInput: AsTestInput<Self>,
    {
        for dir in TestInput::test_dirs() {
            for input in TestInput::collect(self, &dir) {
                f(self, &input)
            }
        }
    }

    fn expect_test_all<TestInput, R>(&self, _f: impl Fn(&Self, &TestInput) -> R)
    where
        R: std::fmt::Debug,
    {
        todo!()
    }

    fn expect_test_all_with_db<TestInput, R>(&self, _f: impl Fn(&Self, &TestInput) -> R)
    where
        R: salsa::DebugWithDb<Self>,
    {
        todo!()
    }

    fn expect_test_all_results<TestInput, T, E>(
        &self,
        _dir: &std::path::Path,
        _f: impl for<'a> Fn(&'a Self, &'a TestInput) -> Result<&'a T, E>,
    ) where
        T: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        todo!()
    }

    fn expect_test_all_results_with_db<TestInput, T, E>(
        &self,
        _dir: &std::path::Path,
        _f: impl for<'a> Fn(&'a Self, &'a TestInput) -> Result<&'a T, E>,
    ) where
        T: salsa::DebugWithDb<Self>,
        E: salsa::DebugWithDb<Self>,
    {
        todo!()
    }
}
