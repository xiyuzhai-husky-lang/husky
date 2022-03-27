use file::FilePtr;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Env {
    Package(FilePtr),
    Module(ScopePtr),
    DatasetConfig,
    Main,
    Morphism,
    Func,
    Proc,
    Test,
    Struct,
    Record,
    Props,
    Enum,
}

impl Env {
    pub fn subscope(&self, db: &dyn AstSalsaQueryGroup, ident: CustomIdentifier) -> ScopePtr {
        match self {
            Env::Package(main) => db
                .subscope(db.module(*main).unwrap(), ident, vec![])
                .unwrap(),
            Env::Module(_) => todo!(),
            Env::DatasetConfig => todo!(),
            Env::Main => todo!(),
            Env::Morphism => todo!(),
            Env::Func => todo!(),
            Env::Proc => todo!(),
            Env::Test => todo!(),
            Env::Struct => todo!(),
            Env::Enum => todo!(),
            Env::Record => todo!(),
            Env::Props => todo!(),
        }
    }
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Env::Package(_) => "package",
            Env::Module(_) => "module",
            Env::DatasetConfig => "dataset config",
            Env::Main => "main",
            Env::Morphism => "def",
            Env::Func => "func",
            Env::Proc => "proc",
            Env::Test => "test",
            Env::Struct => "struct",
            Env::Enum => "enum",
            Env::Record => "record",
            Env::Props => "props",
        })
    }
}
