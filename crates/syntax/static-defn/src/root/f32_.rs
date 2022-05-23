use super::*;

pub static F32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "f32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "f32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[
            &F32_MIN, &F32_MAX, &F32_SGN, &F32_ABS, &F32_SQRT, &F32_COS, &F32_SIN, &F32_TAN,
            &F32_ACOS, &F32_ASIN, &F32_ATAN,
        ],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static F32_MIN: EntityStaticDefn = EntityStaticDefn {
    name: "min",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[StaticInputParameter {
            name: "other",
            contract: InputLiason::Pure,
            ty: "f32",
        }],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_MAX: EntityStaticDefn = EntityStaticDefn {
    name: "max",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[StaticInputParameter {
            name: "other",
            contract: InputLiason::Pure,
            ty: "f32",
        }],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_SGN: EntityStaticDefn = EntityStaticDefn {
    name: "sgn",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_ABS: EntityStaticDefn = EntityStaticDefn {
    name: "abs",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_SQRT: EntityStaticDefn = EntityStaticDefn {
    name: "sqrt",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_COS: EntityStaticDefn = EntityStaticDefn {
    name: "cos",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_SIN: EntityStaticDefn = EntityStaticDefn {
    name: "sin",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_TAN: EntityStaticDefn = EntityStaticDefn {
    name: "tan",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_ACOS: EntityStaticDefn = EntityStaticDefn {
    name: "acos",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_ASIN: EntityStaticDefn = EntityStaticDefn {
    name: "asin",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static F32_ATAN: EntityStaticDefn = EntityStaticDefn {
    name: "atan",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "f32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| todo!(),
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};
