mod b32;
mod f32_;
mod i32_;
mod std_;
mod vec;

pub use b32::*;
pub use f32_::*;
pub use i32_::*;
pub use std_::*;
pub use vec::*;

use dev_utils::{dev_src, static_dev_src};

use crate::*;
use entity_kind::{RoutineKind, TyKind};
use visual_syntax::{trivial_visualizer, StaticVisualTy, StaticVisualizer};
use vm::*;

pub static CLONE_TRAIT_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Clone",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Trait {
        base_route: "Clone",
        members: &[EntityStaticDefn {
            name: "clone",
            subscopes: &[],
            variant: EntityStaticDefnVariant::Method {
                this_liason: ParameterLiason::Pure,
                parameters: &[],
                output_ty: "This",
                generic_parameters: &[],
                kind: MethodStaticDefnVariant::TraitMethod {
                    opt_default_source: Some(LinkageSource::Transfer(linkage!(
                        |values| Ok(values[0].clone_into_stack()),
                        1
                    ))),
                },
                output_liason: OutputLiason::Transfer,
            },
            dev_src: static_dev_src!(),
        }],
        generic_parameters: &[],
    },
    dev_src: static_dev_src!(),
};

pub static VOID_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "void",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "void",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &trivial_visualizer(StaticVisualTy::Void),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B64_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b64",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "b64",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &trivial_visualizer(StaticVisualTy::B64),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static BOOL_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "bool",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "bool",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &trivial_visualizer(StaticVisualTy::Bool),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
