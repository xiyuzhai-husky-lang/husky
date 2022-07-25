mod function;
pub mod utils;

pub use function::*;

use entity_kind::{EntityKind, FieldKind, MemberKind, RoutineKind, TyKind};
use husky_dev_utils::__StaticDevSource;
use husky_liason_semantics::{MemberLiason, OutputLiason, ParameterLiason};
use husky_visual_syntax::StaticVisualTy;
use husky_vm_interface::__Linkage;
use husky_word::RootIdentifier;
use vm::__LinkageFp;

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
pub enum StaticVariadicTemplate {
    None,
    SingleTyped { ty: &'static str },
}

#[derive(PartialEq, Eq)]
pub enum EntityStaticDefnVariant {
    Function {
        spatial_parameters: &'static [StaticSpatialParameter],
        parameters: &'static [StaticParameter],
        variadic_template: StaticVariadicTemplate,
        output_ty: &'static str,
        output_liason: OutputLiason,
        linkage: __Linkage,
    },
    Ty {
        base_route: &'static str,
        spatial_parameters: &'static [StaticSpatialParameter],
        static_trait_impls: &'static [StaticTraitImplDefn],
        ty_members: &'static [&'static EntityStaticDefn],
        variants: &'static [EntityStaticDefn],
        kind: TyKind,
        visual_ty: StaticVisualTy,
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
        liason: MemberLiason,
        ty: &'static str,
        linkage: __Linkage,
    },
    Method {
        this_liason: ParameterLiason,
        parameters: &'static [StaticParameter],
        output_ty: &'static str,
        output_liason: OutputLiason,
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
}

impl EntityStaticDefnVariant {
    pub fn entity_kind(&self) -> EntityKind {
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
            EntityStaticDefnVariant::TraitAssociatedType { .. } => EntityKind::Type(TyKind::Other),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
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
    ($name: expr, $ty: expr) => {
        EntityStaticDefn {
            dev_src: husky_dev_utils::__static_dev_src!(),
            name: $name,
            items: &[],
            variant: EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: $ty },
        }
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MethodStaticDefnKind {
    TypeMethod,
    TraitMethod,
    TraitMethodImpl,
}
