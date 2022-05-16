mod list;
mod static_std;

pub use list::*;
pub use static_std::*;

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
                output_contract: OutputContract::Transfer,
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

pub static F32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "f32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "f32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[
            EntityStaticDefn {
                name: "min",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[StaticInputParameter {
                        name: "other",
                        contract: InputContract::Pure,
                        ty: "f32",
                    }],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "max",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[StaticInputParameter {
                        name: "other",
                        contract: InputContract::Pure,
                        ty: "f32",
                    }],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "sgn",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "i32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "sqrt",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "cos",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "sin",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "tan",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "acos",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "asin",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "atan",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "f32",
                    output_contract: OutputContract::Transfer,
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: |values| todo!(),
                            nargs: 2,
                        }),
                    },
                },
                dev_src: static_dev_src!(),
            },
        ],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "b32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[EntityStaticDefn {
            name: "trailing_zeros",
            subscopes: &[],
            variant: EntityStaticDefnVariant::Method {
                this_contract: InputContract::Pure,
                input_parameters: &[],
                output_ty: "i32",
                output_contract: OutputContract::Transfer,
                generic_parameters: &[],
                kind: MethodStaticDefnKind::TypeMethod {
                    source: LinkageSource::Transfer(Linkage {
                        call: |values| {
                            Ok(StackValue::Primitive(
                                (values[0].as_primitive().as_b32().trailing_zeros() as i32).into(),
                            ))
                        },
                        nargs: 1,
                    }),
                },
            },
            dev_src: static_dev_src!(),
        }],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            compiled: |value| {
                let value: &u32 = value.downcast_ref();
                VisualProps::Primitive {
                    value: (*value).into(),
                }
            },
        },
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
