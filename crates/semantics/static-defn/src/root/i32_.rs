use super::*;

pub static I32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "i32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "i32",
        generic_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[&I32_MIN, &I32_MAX, &I32_SGN, &I32_ABS],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static I32_MIN: EntityStaticDefn = EntityStaticDefn {
    name: "min",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "other",
            contract: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(|values| todo!(), 2)),
        },
    },
    dev_src: static_dev_src!(),
};

pub static I32_MAX: EntityStaticDefn = EntityStaticDefn {
    name: "max",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "other",
            contract: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(|values| todo!(), 2)),
        },
    },
    dev_src: static_dev_src!(),
};

pub static I32_SGN: EntityStaticDefn = EntityStaticDefn {
    name: "sgn",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(|values| todo!(), 1)),
        },
    },
    dev_src: static_dev_src!(),
};

pub static I32_ABS: EntityStaticDefn = EntityStaticDefn {
    name: "abs",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(
                |values| {
                    Ok(TempValue::Copyable(
                        values[0].take_copyable().take_i32().abs().into(),
                    ))
                },
                1
            )),
        },
    },
    dev_src: static_dev_src!(),
};
