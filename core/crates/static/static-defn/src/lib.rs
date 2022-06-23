mod call;
mod ty;
pub mod utils;

pub use call::*;
pub use ty::*;

use dev_utils::StaticDevSource;
use entity_kind::{EntityKind, FieldKind, MemberKind, RoutineKind, TyKind};
use liason::{MemberLiason, OutputLiason, ParameterLiason};
use visual_syntax::StaticVisualizer;
use vm::Linkage;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityStaticDefn {
    pub name: &'static str,
    pub items: &'static [&'static EntityStaticDefn],
    pub variant: EntityStaticDefnVariant,
    pub dev_src: StaticDevSource,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EntityStaticDefnVariant {
    Routine {
        generic_parameters: &'static [StaticGenericPlaceholder],
        parameters: Vec<StaticParameter>,
        output_ty: &'static str,
        output_liason: OutputLiason,
        linkage: Linkage,
        paradigm: RoutineKind,
    },
    Morphism,
    Ty {
        base_route: &'static str,
        generic_parameters: &'static [StaticGenericPlaceholder],
        static_trait_impls: &'static [StaticTraitImplDefn],
        ty_members: &'static [&'static EntityStaticDefn],
        variants: &'static [EntityStaticDefn],
        kind: TyKind,
        visualizer: &'static StaticVisualizer,
        opt_type_call: Option<&'static EntityStaticDefn>,
    },
    Trait {
        base_route: &'static str,
        generic_parameters: &'static [StaticGenericPlaceholder],
        members: &'static [EntityStaticDefn],
    },
    Module,
    TyField {
        field_kind: FieldKind,
        liason: MemberLiason,
        ty: &'static str,
        static_linkage_source: &'static LinkageSource,
    },
    Method {
        this_liason: ParameterLiason,
        parameters: &'static [StaticParameter],
        output_ty: &'static str,
        output_liason: OutputLiason,
        generic_parameters: &'static [StaticGenericPlaceholder],
        kind: MethodStaticDefnVariant,
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
            EntityStaticDefnVariant::Routine { .. } => EntityKind::Function { is_lazy: false },
            EntityStaticDefnVariant::Ty { kind, .. } => EntityKind::Type(*kind),
            EntityStaticDefnVariant::Module => EntityKind::Module,
            EntityStaticDefnVariant::Trait { .. } => EntityKind::Trait,
            EntityStaticDefnVariant::Method { .. } => {
                EntityKind::Member(MemberKind::Method { is_lazy: false })
            }
            EntityStaticDefnVariant::TraitAssociatedType { .. } => EntityKind::Type(TyKind::Other),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
            EntityStaticDefnVariant::Morphism => todo!(),
        }
    }
}
