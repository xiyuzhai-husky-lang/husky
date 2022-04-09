use entity_syntax::RoutineKind;
use file::FilePtr;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AstContext {
    Package(FilePtr),
    Module(EntityRoutePtr),
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

impl AstContext {
    pub fn subscope(&self, db: &dyn AstSalsaQueryGroup, ident: CustomIdentifier) -> EntityRoutePtr {
        match self {
            AstContext::Package(main) => db
                .subscope(db.module(*main).unwrap(), ident, vec![])
                .unwrap(),
            AstContext::Module(_) => todo!(),
            AstContext::DatasetConfig => todo!(),
            AstContext::Main => todo!(),
            AstContext::Morphism => todo!(),
            AstContext::Func => todo!(),
            AstContext::Proc => todo!(),
            AstContext::Test => todo!(),
            AstContext::Struct => todo!(),
            AstContext::Enum => todo!(),
            AstContext::Record => todo!(),
            AstContext::Props => todo!(),
        }
    }
}

impl From<RoutineKind> for AstContext {
    fn from(routine_kind: RoutineKind) -> Self {
        match routine_kind {
            RoutineKind::Test => AstContext::Test,
            RoutineKind::Proc => AstContext::Proc,
            RoutineKind::Func => AstContext::Func,
        }
    }
}

impl std::fmt::Display for AstContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AstContext::Package(_) => "pack",
            AstContext::Module(_) => "module",
            AstContext::DatasetConfig => "dataset config",
            AstContext::Main => "main",
            AstContext::Morphism => "def",
            AstContext::Func => "func",
            AstContext::Proc => "proc",
            AstContext::Test => "test",
            AstContext::Struct => "struct",
            AstContext::Enum => "enum",
            AstContext::Record => "record",
            AstContext::Props => "props",
        })
    }
}
