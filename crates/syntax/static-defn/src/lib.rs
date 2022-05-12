mod call;
mod root;
mod ty;

pub use call::*;
use dev_utils::StaticDevSource;
pub use root::*;
pub use ty::*;

use entity_kind::{EntityKind, MemberKind, RoutineContextKind, RoutineKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputContract, Linkage, OutputContract};

#[derive(Debug, PartialEq, Eq)]
pub struct EntityStaticDefn {
    pub name: &'static str,
    pub subscopes: &'static [(&'static str, &'static EntityStaticDefn)],
    pub variant: EntityStaticDefnVariant,
    pub dev_src: StaticDevSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntityStaticDefnVariant {
    Routine {
        generic_placeholders: &'static [StaticGenericPlaceholder],
        input_placeholders: Vec<StaticInputPlaceholder>,
        output_ty: &'static str,
        output_contract: OutputContract,
        linkage: Linkage,
        routine_kind: RoutineKind,
    },
    Type {
        base_route: &'static str,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        trait_impls: &'static [StaticTraitImplDefn],
        type_members: &'static [EntityStaticDefn],
        variants: &'static [EntityStaticDefn],
        kind: TyKind,
        visualizer: StaticVisualizer,
        opt_type_call: Option<&'static EntityStaticDefn>,
    },
    Trait {
        base_route: &'static str,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        members: &'static [EntityStaticDefn],
    },
    Module,
    TypeField {
        field_variant: StaticFieldVariant,
    },
    Method {
        this_contract: InputContract,
        input_placeholders: &'static [StaticInputPlaceholder],
        output_ty: &'static str,
        output_contract: OutputContract,
        generic_placeholders: &'static [StaticGenericPlaceholder],
        kind: MethodStaticDefnKind,
    },
    TraitAssociatedType {
        trai: &'static str,
        traits: &'static [&'static str],
    },
    TraitAssociatedTypeImpl {
        ty: &'static str,
    },
    TraitAssociatedConstSize,
}

impl EntityStaticDefnVariant {
    pub fn entity_kind(&self) -> EntityKind {
        match self {
            EntityStaticDefnVariant::Routine { .. } => EntityKind::Routine,
            EntityStaticDefnVariant::Type { kind, .. } => EntityKind::Type(*kind),
            EntityStaticDefnVariant::Module => EntityKind::Module,
            EntityStaticDefnVariant::Trait { .. } => EntityKind::Trait,
            EntityStaticDefnVariant::Method { .. } => EntityKind::Member(MemberKind::Method),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => EntityKind::Type(TyKind::Other),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TypeField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
        }
    }
}
