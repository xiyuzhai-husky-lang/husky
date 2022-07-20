use super::*;

pub trait __F32X {
    fn sgnx(self) -> i32;
}

impl __F32X for f32 {
    fn sgnx(self) -> i32 {
        {
            if self > 0. {
                1
            } else if self == 0. {
                0
            } else {
                -1
            }
        }
    }
}

pub static F32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "f32",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "f32",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[
            &F32_MIN, &F32_MAX, &F32_SGN, &F32_ABS, &F32_SQRT, &F32_COS, &F32_SIN, &F32_TAN,
            &F32_ACOS, &F32_ASIN, &F32_ATAN,
        ],
        variants: &[],
        kind: TyKind::Primitive,
        visual_ty: StaticVisualTy::F32,
        opt_type_call: None,
    },
    dev_src: __static_dev_src!(),
};

pub static F32_MIN: EntityStaticDefn = EntityStaticDefn {
    name: "min",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "other",
            liason: ParameterLiason::Pure,
            ty: "f32",
        }],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            let this = values[0].take_copyable().take_f32();
            let other = values[0].take_copyable().take_f32();
            __TempValue::Copyable(this.min(other).into())
        }, some f32::min)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_MAX: EntityStaticDefn = EntityStaticDefn {
    name: "max",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "other",
            liason: ParameterLiason::Pure,
            ty: "f32",
        }],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            let this = values[0].take_copyable().take_f32();
            let other = values[0].take_copyable().take_f32();
            __TempValue::Copyable(this.max(other).into())
        }, some f32::max)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_SGN: EntityStaticDefn = EntityStaticDefn {
    name: "sgnx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            let f = values[0].take_copyable().take_f32();
            __TempValue::Copyable(f.sgnx().into())
        }, some f32::sgnx)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_ABS: EntityStaticDefn = EntityStaticDefn {
    name: "abs",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().abs().into())
        }, some f32::abs)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_SQRT: EntityStaticDefn = EntityStaticDefn {
    name: "sqrt",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().sqrt().into())
        }, some f32::sqrt)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_COS: EntityStaticDefn = EntityStaticDefn {
    name: "cos",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().cos().into())
        }, some f32::cos)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_SIN: EntityStaticDefn = EntityStaticDefn {
    name: "sin",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().sin().into())
        }, some f32::sin)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_TAN: EntityStaticDefn = EntityStaticDefn {
    name: "tan",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().tan().into())
        }, some f32::tan)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_ACOS: EntityStaticDefn = EntityStaticDefn {
    name: "acos",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().acos().into())
        }, some f32::acos)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_ASIN: EntityStaticDefn = EntityStaticDefn {
    name: "asin",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().asin().into())
        }, some f32::asin)),
    },
    dev_src: __static_dev_src!(),
};

pub static F32_ATAN: EntityStaticDefn = EntityStaticDefn {
    name: "atan",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(specific_transfer_linkage!(|_, values| {
            __TempValue::Copyable(values[0].take_copyable().take_f32().atan().into())
        }, some f32::atan)),
    },
    dev_src: __static_dev_src!(),
};
