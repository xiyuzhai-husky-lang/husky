use husky_print_utils::p;
use husky_trace_protocol::VisualData;

use super::*;

pub static F32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "f32",
    items: &[],
    variant: EntityStaticDefnVariant::EtherealTerm {
        base_route: "f32",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[
            &F32_MIN, &F32_MAX, &F32_SGN, &F32_ABS, &F32_SQRT, &F32_COS, &F32_SIN, &F32_TAN,
            &F32_ACOS, &F32_ASIN, &F32_ATAN,
        ],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::Float,
            fp: StaticVisualizerFp(|value| {
                Ok(VisualData::Primitive {
                    value: value.downcast_f32().into(),
                })
            }),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static F32_MIN: EntityStaticDefn = EntityStaticDefn {
    name: "min",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[StaticParameter {
            name: "other",
            modifier: ParameterModifier::None,
            ty: "f32",
        }],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| {
                let this = values[0].downcast_f32();
                let other = values[1].downcast_f32();
                this.min(other).to_register()
            },
            some base f32::min as fn (f32,f32)-> f32
        )),
    },
    dev_src: static_dev_src!(),
};

pub static F32_MAX: EntityStaticDefn = EntityStaticDefn {
    name: "max",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[StaticParameter {
            name: "other",
            modifier: ParameterModifier::None,
            ty: "f32",
        }],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
            let this = values[0].downcast_f32();
            let other = values[1].downcast_f32();
             this.max(other) .to_register() 
        }, some base f32::max as fn (f32,f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_SGN: EntityStaticDefn = EntityStaticDefn {
    name: "sgnx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
            let f = values[0].downcast_f32();
             f.sgnx() .to_register() 
        }, some base f32::sgnx as fn (f32)-> i32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_ABS: EntityStaticDefn = EntityStaticDefn {
    name: "abs",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
             values[0].downcast_f32().abs() .to_register() 
        }, some base f32::abs as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_SQRT: EntityStaticDefn = EntityStaticDefn {
    name: "sqrt",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
             values[0].downcast_f32().sqrt().to_register()
        }, some base f32::sqrt as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_COS: EntityStaticDefn = EntityStaticDefn {
    name: "cos",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
            values[0].downcast_f32().cos().to_register()
        }, some base f32::cos as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_SIN: EntityStaticDefn = EntityStaticDefn {
    name: "sin",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
             values[0].downcast_f32().sin() .to_register() 
        }, some base f32::sin as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_TAN: EntityStaticDefn = EntityStaticDefn {
    name: "tan",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
             values[0].downcast_f32().tan() .to_register() 
        }, some base f32::tan as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_ACOS: EntityStaticDefn = EntityStaticDefn {
    name: "acos",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
            todo!();
            values[0].downcast_f32().acos().to_register()
        }, some base f32::acos as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_ASIN: EntityStaticDefn = EntityStaticDefn {
    name: "asin",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
             values[0].downcast_f32().asin() .to_register() 
        }, some base f32::asin as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};

pub static F32_ATAN: EntityStaticDefn = EntityStaticDefn {
    name: "atan",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        return_ty: "f32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(|values, _| {
             values[0].downcast_f32().atan() .to_register() 
        }, some base f32::atan as fn (f32)-> f32)),
    },
    dev_src: static_dev_src!(),
};
