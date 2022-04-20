mod call;
mod root;
mod trai;
mod ty;

pub use call::*;
use dev_utils::StaticDevSource;
pub use root::*;
pub use trai::*;
pub use ty::*;

use entity_kind::EntityKind;

#[derive(Debug, PartialEq, Eq)]
pub struct StaticEntityDefn {
    pub name: &'static str,
    pub subscopes: &'static [(&'static str, &'static StaticEntityDefn)],
    pub variant: StaticEntityDefnVariant,
    pub dev_src: StaticDevSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticEntityDefnVariant {
    Func(StaticCallDefn),
    Type(&'static StaticTypeDefn),
    Trait(&'static StaticTraitDecl),
    Module,
}

impl StaticEntityDefnVariant {
    pub fn raw_entity_kind(&self) -> EntityKind {
        match self {
            StaticEntityDefnVariant::Func(_) => EntityKind::Routine,
            StaticEntityDefnVariant::Type(ty_decl) => EntityKind::Type(ty_decl.kind),
            StaticEntityDefnVariant::Module => EntityKind::Module,
            StaticEntityDefnVariant::Trait(trait_decl) => EntityKind::Trait,
        }
    }
}
