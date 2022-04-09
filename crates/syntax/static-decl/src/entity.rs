use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum StaticEntityDecl {
    Func(StaticFuncDecl),
    Ty {
        raw_ty_kind: TyKind,
        visualizer: StaticVisualizer,
    },
    TyTemplate,
    Trait(StaticTraitDecl),
    Module,
}

impl StaticEntityDecl {
    pub fn raw_entity_kind(&self) -> EntityKind {
        match self {
            StaticEntityDecl::Func(_) => EntityKind::Routine,
            StaticEntityDecl::Ty { raw_ty_kind, .. } => EntityKind::Type(*raw_ty_kind),
            StaticEntityDecl::Module => EntityKind::Module,
            StaticEntityDecl::TyTemplate => EntityKind::Type(TyKind::Vec),
            StaticEntityDecl::Trait { .. } => todo!(),
        }
    }
}
