mod b32;
mod f32_;
mod static_std;
mod vec;

pub use b32::*;
pub use f32_::*;
pub use static_std::*;
pub use vec::*;

use dev_utils::{dev_src, static_dev_src};

use crate::*;
use entity_kind::{RoutineKind, TyKind};
use visual_syntax::{StaticVisualizer, VisualProps, TRIVIAL_VISUALIZER};
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
                this_contract: vm::InputContract::Pure,
                input_parameters: &[],
                output_ty: "This",
                generic_parameters: &[],
                kind: MethodStaticDefnKind::TraitMethod {
                    opt_default_source: Some(LinkageSource::Transfer(Linkage {
                        call: |values| Ok(values[0].clone_into_stack()),
                        nargs: 1,
                    })),
                },
                output_contract: OutputLiason::Transfer,
            },
            dev_src: static_dev_src!(),
        }],
        generic_placeholders: &[],
    },
    dev_src: static_dev_src!(),
};

pub static VOID_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "void",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "void",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static I32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "i32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "i32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B64_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b64",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "b64",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static BOOL_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "bool",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "bool",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};
