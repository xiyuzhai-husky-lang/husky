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
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticGenericPlaceholderVariant {
    Const,
    Type { traits: &'static [&'static str] },
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionStaticDefnVariant {
    Model(ModelLinkage),
    GenericTransfer(GenericRoutineLinkage),
    Routine {
        __Linkage: __SpecificRoutineLinkage,
        routine_kind: RoutineKind,
    },
}

impl FunctionStaticDefnVariant {
    pub fn requires_lazy(&self) -> bool {
        match self {
            FunctionStaticDefnVariant::Model(_) => true,
            FunctionStaticDefnVariant::GenericTransfer(_) => false,
            FunctionStaticDefnVariant::Routine { .. } => false,
        }
    }
}
