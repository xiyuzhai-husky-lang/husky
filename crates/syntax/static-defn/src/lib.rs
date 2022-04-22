mod call;
mod root;
mod ty;

pub use call::*;
use dev_utils::StaticDevSource;
pub use root::*;
pub use ty::*;

use entity_kind::{EntityKind, TyKind};
use visual_syntax::StaticVisualizer;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityStaticDefn {
    pub name: &'static str,
    pub subscopes: &'static [(&'static str, &'static EntityStaticDefn)],
    pub variant: StaticEntityDefnVariant,
    pub dev_src: StaticDevSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StaticEntityDefnVariant {
    Func(StaticCallDefn),
    Type {
        base_route: &'static str,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        trait_impls: &'static [StaticTraitImplDefn],
        type_members: &'static [TypeMemberStaticDefn],
        variants: &'static [StaticEnumVariantDecl],
        kind: TyKind,
        visualizer: StaticVisualizer,
        opt_type_call: Option<&'static StaticCallDefn>,
    },
    Trait {
        base_route: &'static str,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        members: &'static [TraitMemberStaticDefn],
    },
    Module,
}

impl StaticEntityDefnVariant {
    pub fn raw_entity_kind(&self) -> EntityKind {
        match self {
            StaticEntityDefnVariant::Func(_) => EntityKind::Routine,
            StaticEntityDefnVariant::Type { kind, .. } => EntityKind::Type(*kind),
            StaticEntityDefnVariant::Module => EntityKind::Module,
            StaticEntityDefnVariant::Trait { .. } => EntityKind::Trait,
        }
    }
}
