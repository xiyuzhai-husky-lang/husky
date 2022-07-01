use husky_trace_protocol::*;
use visual_syntax::{StaticVisualTy, StaticVisualizerVariant};

use super::*;

pub static B32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b32",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "b32",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[&B32_LEADING_ZEROS, &B32_TRAILING_ZEROS, &B32_LAST_BITS],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &StaticVisualizer {
            ty: StaticVisualTy::B32,
            variant: StaticVisualizerVariant::Compiled {
                call: |value| {
                    let value: &u32 = value.downcast_ref();
                    VisualData::Primitive {
                        value: (*value).into(),
                    }
                },
            },
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B32_LEADING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "leading_zeros",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::SpecificTransfer(routine_linkage!(
            |values| {
                Ok(TempValue::Copyable(
                    (values[0].take_copyable().take_b32().leading_zeros() as i32).into(),
                ))
            },
            1
        ))),
    },
    dev_src: static_dev_src!(),
};

pub static B32_TRAILING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "trailing_zeros",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::SpecificTransfer(routine_linkage!(
            |values| {
                Ok(TempValue::Copyable(
                    (values[0].take_copyable().take_b32().trailing_zeros() as i32).into(),
                ))
            },
            1
        ))),
    },
    dev_src: static_dev_src!(),
};

pub static B32_LAST_BITS: EntityStaticDefn = EntityStaticDefn {
    name: "last_bits",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "k",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "b32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(Linkage::SpecificTransfer(routine_linkage!(
            |values| {
                let b = values[0].take_copyable().take_b32();
                let i = values[1].take_copyable().take_i32();
                let last_bits = b & ((1 << i) - 1);
                Ok(TempValue::Copyable(last_bits.into()))
            },
            2
        ))),
    },
    dev_src: static_dev_src!(),
};
