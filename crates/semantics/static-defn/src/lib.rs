mod call;
mod root;
mod ty;

pub use call::*;
use dev_utils::StaticDevSource;
pub use root::*;
pub use ty::*;

use entity_kind::{EntityKind, FieldKind, MemberKind, RoutineKind, TyKind};
use visual_syntax::StaticVisualizer;
use vm::{InputLiason, Linkage, MemberLiason, OutputLiason};

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
        generic_parameters: &'static [StaticGenericPlaceholder],
        parameters: Vec<StaticInputParameter>,
        output_ty: &'static str,
        output_liason: OutputLiason,
        linkage: Linkage,
        routine_kind: RoutineKind,
    },
    Ty {
        base_route: &'static str,
        generic_parameters: &'static [StaticGenericPlaceholder],
        static_trait_impls: &'static [StaticTraitImplDefn],
        ty_members: &'static [&'static EntityStaticDefn],
        variants: &'static [EntityStaticDefn],
        kind: TyKind,
        visualizer: StaticVisualizer,
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
    },
    Method {
        this_contract: InputLiason,
        input_parameters: &'static [StaticInputParameter],
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
            EntityStaticDefnVariant::Method { .. } => EntityKind::Member(MemberKind::Method),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => EntityKind::Type(TyKind::Other),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
        }
    }
}
