use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Env {
    Package,
    Module(ScopePtr),
    DatasetConfig,
    Main,
    Def,
    Func,
    Proc,
    Test,
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Env::Package => "package",
            Env::Module(scope) => "module",
            Env::DatasetConfig => "dataset config",
            Env::Main => "main",
            Env::Def => "def",
            Env::Func => "func",
            Env::Proc => "proc",
            Env::Test => "test",
        })
    }
}
