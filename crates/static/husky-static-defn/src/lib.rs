mod function;
pub mod utils;

pub use function::*;

use husky_dev_utils::__StaticDevSource;
use husky_entity_kind::{EntityKind, FieldKind, MemberKind, RoutineKind, TyKind};
use husky_liason_semantics::{MemberModifier, OutputModifier, ParameterModifier};
use husky_static_visualizer::{StaticVisualTy, StaticVisualizer};
use husky_vm::__ResolvedLinkage;
use husky_vm_interface::__Linkage;
use husky_word::RootIdentifier;

pub trait ResolveStaticRootDefn {
    fn __root_defn_resolver(&self) -> fn(ident: RootIdentifier) -> &'static EntityStaticDefn;
}

#[derive(PartialEq, Eq)]
pub struct EntityStaticDefn {
    pub name: &'static str,
    pub items: &'static [&'static EntityStaticDefn],
    pub variant: EntityStaticDefnVariant,
    pub dev_src: __StaticDevSource,
}

impl std::fmt::Debug for EntityStaticDefn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityStaticDefn")
            .field("name", &self.name)
            .field("items", &self.items)
            .field("dev_src", &self.dev_src)
            .finish()
    }
}

#[derive(PartialEq, Eq)]
pub enum StaticVariadicParameterDecl {
    None,
    RepeatSingle(StaticParameter),
}

#[derive(PartialEq, Eq)]
pub enum EntityStaticDefnVariant {
    Function {
        spatial_parameters: &'static [StaticSpatialParameter],
        parameters: &'static [StaticParameter],
        variadic_template: StaticVariadicParameterDecl,
        output_ty: &'static str,
        output_liason: OutputModifier,
        linkage: __Linkage,
    },
    Ty {
        base_route: &'static str,
        spatial_parameters: &'static [StaticSpatialParameter],
        trait_impls: &'static [StaticTraitImplDefn],
        ty_members: &'static [&'static EntityStaticDefn],
        variants: &'static [EntityStaticDefn],
        kind: TyKind,
        visualizer: StaticVisualizer,
        opt_type_call: Option<&'static EntityStaticDefn>,
    },
    Trait {
        base_route: &'static str,
        spatial_parameters: &'static [StaticSpatialParameter],
        members: &'static [EntityStaticDefn],
    },
    Module,
    TyField {
        field_kind: FieldKind,
        liason: MemberModifier,
        field_ty: &'static str,
        linkage: __Linkage,
    },
    Method {
        this_modifier: ParameterModifier,
        parameters: &'static [StaticParameter],
        output_ty: &'static str,
        output_liason: OutputModifier,
        spatial_parameters: &'static [StaticSpatialParameter],
        method_static_defn_kind: MethodStaticDefnKind,
        opt_linkage: Option<__Linkage>,
    },
    TraitAssociatedType {
        trai: &'static str,
        traits: &'static [&'static str],
    },
    TraitAssociatedTypeImpl {
        ty: &'static str,
    },
    TraitAssociatedConstSize,
    EnumVariant,
}

impl EntityStaticDefnVariant {
    pub fn husky_entity_kind(&self) -> EntityKind {
        match self {
            EntityStaticDefnVariant::Function { ref linkage, .. } => EntityKind::Function {
                requires_lazy: linkage.requires_lazy(),
            },
            EntityStaticDefnVariant::Ty { kind, .. } => EntityKind::Type(*kind),
            EntityStaticDefnVariant::Module => EntityKind::Module,
            EntityStaticDefnVariant::Trait { .. } => EntityKind::Trait,
            EntityStaticDefnVariant::Method { .. } => {
                EntityKind::Member(MemberKind::Method { is_lazy: false })
            }
            EntityStaticDefnVariant::TraitAssociatedType { .. } => {
                EntityKind::Type(TyKind::AssociatedAny)
            }
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
            EntityStaticDefnVariant::EnumVariant => EntityKind::EnumVariant,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct StaticTraitImplDefn {
    pub trai: &'static str,
    pub member_impls: &'static [EntityStaticDefn],
    pub dev_src: __StaticDevSource,
}

#[macro_export]
macro_rules! associated_type_impl {
    ($name: expr, $ty: expr) => {{
        EntityStaticDefn {
            dev_src: husky_dev_utils::static_dev_src!(),
            name: $name,
            items: &[],
            variant: EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: $ty },
        }
    }};
}
#[macro_export]
macro_rules! clone_trait_impl {
    ($this_ty_route: expr) => {{
        StaticTraitImplDefn {
            dev_src: static_dev_src!(),
            trai: "Clone",
            member_impls: &[EntityStaticDefn {
                dev_src: husky_dev_utils::static_dev_src!(),
                name: "clone",
                items: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_modifier: ParameterModifier::None,
                    parameters: &[],
                    output_ty: $this_ty_route,
                    output_liason: OutputModifier::Transfer,
                    spatial_parameters: &[],
                    method_static_defn_kind: MethodStaticDefnKind::TraitMethodImpl,
                    opt_linkage: None,
                },
            }],
        }
    }};
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodStaticDefnKind {
    TypeMethod,
    TraitMethod,
    TraitMethodImpl,
}
