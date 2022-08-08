use crate::*;
use vm::*;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticParameter {
    pub name: &'static str,
    pub liason: ParameterLiason,
    pub ty: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaticSpatialParameter {
    pub name: &'static str,
    pub variant: StaticGenericPlaceholderVariant,
    pub dev_src: __StaticDevSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticGenericPlaceholderVariant {
    Const,
    Type { traits: &'static [&'static str] },
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionStaticDefnVariant {
    Model(__ModelLinkage),
    Routine {
        __Linkage: __LinkageFp,
        routine_kind: RoutineKind,
    },
}

impl FunctionStaticDefnVariant {
    pub fn requires_lazy(&self) -> bool {
        match self {
            FunctionStaticDefnVariant::Model(_) => true,
            FunctionStaticDefnVariant::Routine { .. } => false,
        }
    }
}
