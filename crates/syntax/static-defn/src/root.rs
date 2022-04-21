mod static_std;

pub use static_std::*;

use crate::*;
use entity_kind::TyKind;
use visual_syntax::{StaticVisualizer, VisualProps, TRIVIAL_VISUALIZER};
use vm::*;

pub static CLONE_TRAIT_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "Clone",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Trait(&CLONE_TRAIT_DECL),
    dev_src: dev_utils::static_dev_src!(),
};

pub static VOID_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "void",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&StaticTypeDefn {
        base_route: "void",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
    dev_src: dev_utils::static_dev_src!(),
};

pub static I32_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "i32",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&StaticTypeDefn {
        base_route: "i32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
    dev_src: dev_utils::static_dev_src!(),
};

pub static F32_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "f32",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&StaticTypeDefn {
        base_route: "f32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
    dev_src: dev_utils::static_dev_src!(),
};

pub static B32_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "b32",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&StaticTypeDefn {
        base_route: "b32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
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
    }),
    dev_src: dev_utils::static_dev_src!(),
};

pub static B64_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "b64",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&StaticTypeDefn {
        base_route: "b64",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
    dev_src: dev_utils::static_dev_src!(),
};

pub static BOOL_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "bool",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&StaticTypeDefn {
        base_route: "bool",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    }),
    dev_src: dev_utils::static_dev_src!(),
};

pub static VEC_TYPE_DEFN: StaticEntityDefn = StaticEntityDefn {
    name: "Vec",
    subscopes: &[],
    variant: StaticEntityDefnVariant::Type(&VEC_TYPE_DECL),
    dev_src: dev_utils::static_dev_src!(),
};

pub static CLONE_TRAIT_DECL: StaticTraitDecl = StaticTraitDecl {
    base_route: "Clone",
    members: &[StaticTraitMemberDecl::Method(StaticMethodDefn {
        name: "clone",
        this_contract: vm::InputContract::Pure,
        inputs: &[],
        output_ty: "This",
        generic_placeholders: &[],
        kind: StaticMethodKind::TraitMethod("Clone"),
        output_contract: OutputContract::Pure,
    })],
    generic_placeholders: &[],
};

pub static VEC_TYPE_DECL: StaticTypeDefn = StaticTypeDefn {
    base_route: "Vec",
    generic_placeholders: &[StaticGenericPlaceholder {
        name: "E",
        variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
    }],
    trait_impls: &[
        StaticTraitImplDefn {
            route: "Clone",
            member_impls: &[],
        },
        StaticTraitImplDefn {
            route: "std::ops::Index<i32>",
            member_impls: &[associated_type!("Output", "E")],
        },
    ],
    type_members: &[
        StaticTypeMemberDefn::Method(StaticMethodDefn {
            name: "len",
            this_contract: InputContract::Pure,
            inputs: &[],
            output_ty: "i32",
            generic_placeholders: &[],
            kind: StaticMethodKind::TypeMethod,
            output_contract: OutputContract::Pure,
        }),
        StaticTypeMemberDefn::Method(StaticMethodDefn {
            name: "push",
            this_contract: InputContract::BorrowMut,
            inputs: &[StaticInputPlaceholder {
                contract: InputContract::Move,
                ty: "E",
                name: "element",
            }],
            output_ty: "void",
            generic_placeholders: &[],
            kind: StaticMethodKind::TypeMethod,
            output_contract: OutputContract::Pure,
        }),
        StaticTypeMemberDefn::Method(StaticMethodDefn {
            name: "pop",
            this_contract: InputContract::BorrowMut,
            inputs: &[],
            output_ty: "E",
            generic_placeholders: &[],
            kind: StaticMethodKind::TypeMethod,
            output_contract: OutputContract::Pure,
        }),
    ],
    variants: &[],
    kind: TyKind::Vec,
    visualizer: TRIVIAL_VISUALIZER,
    opt_type_call: Some(&NEW_VEC_DECL),
};

static NEW_VEC_DECL: StaticCallDefn = StaticCallDefn {
    generic_placeholders: &[],
    inputs: vec![],
    output_ty: "Vec<E>",
    output_contract: OutputContract::Pure,
    linkage: Linkage {
        call: |values| Ok(StackValue::Boxed(BoxedValue::new(Vec::<VirtualTy>::new()))),
        nargs: 0,
    },
};
