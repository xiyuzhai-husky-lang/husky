use husky_trace_protocol::VisualData;

use super::*;

pub static I32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "i32",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "i32",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[&I32_MIN, &I32_MAX, &I32_SGN, &I32_ABS],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Integer,
            fp: StaticVisualizerFp(|value| {
                Ok(VisualData::Primitive {
                    value: value.downcast_i32().into(),
                })
            }),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static I32_MIN: EntityStaticDefn = EntityStaticDefn {
    name: "min",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[StaticParameter {
            name: "other",
            modifier: ParameterModifier::None,
            ty: "i32",
        }],
        return_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| values[0].downcast_i32().min(values[1].downcast_i32()).to_register(),
            some base i32::min as fn (i32, i32)-> i32
        )),
    },
    dev_src: static_dev_src!(),
};

pub static I32_MAX: EntityStaticDefn = EntityStaticDefn {
    name: "max",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[StaticParameter {
            name: "other",
            modifier: ParameterModifier::None,
            ty: "i32",
        }],
        return_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
                |values, _| values[0].downcast_i32().max(values[1].downcast_i32()).to_register(),
                some base i32::max as fn (i32, i32)-> i32)),
    },
    dev_src: static_dev_src!(),
};

pub static I32_SGN: EntityStaticDefn = EntityStaticDefn {
    name: "sgn",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| values[0].downcast_i32().sgn().to_register(),
            some base i32::sgn as fn (i32)-> i32
        )),
    },
    dev_src: static_dev_src!(),
};

pub static I32_ABS: EntityStaticDefn = EntityStaticDefn {
    name: "abs",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
            values[0].downcast_i32().abs().to_register()
        }, some base i32::abs as fn (i32)-> i32)),
    },
    dev_src: static_dev_src!(),
};
