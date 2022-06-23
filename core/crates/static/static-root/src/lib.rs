mod b32;
mod clone;
mod domains;
mod f32_;
mod i32_;
mod std_;
mod vec;

pub use b32::*;
pub use clone::*;
pub use domains::*;
pub use f32_::*;
pub use i32_::*;
pub use std_::*;
pub use vec::*;

use dev_utils::StaticDevSource;
use dev_utils::{dev_src, static_dev_src};
use entity_kind::{EntityKind, FieldKind, MemberKind, RoutineKind, TyKind};
use liason::{MemberLiason, OutputLiason, ParameterLiason};
use static_defn::StaticParameter;
use static_defn::*;
use visual_syntax::{primitive_visualizer, StaticVisualTy, StaticVisualizer};
use vm::*;

pub static VOID_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "void",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "void",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &primitive_visualizer(StaticVisualTy::Void),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B64_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b64",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "b64",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &primitive_visualizer(StaticVisualTy::B64),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static BOOL_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "bool",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "bool",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &primitive_visualizer(StaticVisualTy::Bool),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
