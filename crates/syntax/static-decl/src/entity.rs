use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum StaticEntityDecl {
    Func(StaticCallDecl),
    Type(&'static StaticTypeDecl),
    Trait(&'static StaticTraitDecl),
    Module,
}

impl StaticEntityDecl {
    pub fn raw_entity_kind(&self) -> EntityKind {
        match self {
            StaticEntityDecl::Func(_) => EntityKind::Routine,
            StaticEntityDecl::Type(ty_decl) => EntityKind::Type(ty_decl.kind),
            StaticEntityDecl::Module => EntityKind::Module,
            StaticEntityDecl::Trait { .. } => todo!(),
        }
    }
}
