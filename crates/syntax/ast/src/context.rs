use entity_kind::RoutineContextKind;
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
    Enum(EntityRoutePtr),
}

impl AstContext {
    pub fn child_route(
        &self,
        db: &dyn AstSalsaQueryGroup,
        ident: CustomIdentifier,
    ) -> EntityRoutePtr {
        match self {
            AstContext::Package(main) => db
                .child_route(db.module(*main).unwrap(), ident, vec![])
                .unwrap(),
            AstContext::Module(route) => db.child_route(*route, ident, vec![]).unwrap(),
            AstContext::DatasetConfig => todo!(),
            AstContext::Main => todo!(),
            AstContext::Morphism => todo!(),
            AstContext::Func => todo!(),
            AstContext::Proc => todo!(),
            AstContext::Test => todo!(),
            AstContext::Struct => todo!(),
            AstContext::Enum(_) => todo!(),
            AstContext::Record => todo!(),
            AstContext::Props => todo!(),
        }
    }
}

impl From<RoutineContextKind> for AstContext {
    fn from(routine_kind: RoutineContextKind) -> Self {
        match routine_kind {
            RoutineContextKind::Test => AstContext::Test,
            RoutineContextKind::Proc => AstContext::Proc,
            RoutineContextKind::Func => AstContext::Func,
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
            AstContext::Enum(_) => "enum",
            AstContext::Record => "record",
            AstContext::Props => "props",
        })
    }
}
