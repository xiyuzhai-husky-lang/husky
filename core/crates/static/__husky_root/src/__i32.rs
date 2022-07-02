use super::*;

pub static I32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "i32",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "i32",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[&I32_MIN, &I32_MAX, &I32_SGN, &I32_ABS],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: &primitive_visualizer(StaticVisualTy::I32),
        opt_type_call: None,
    },
    dev_src: __static_dev_src!(),
};

pub static I32_MIN: EntityStaticDefn = EntityStaticDefn {
    name: "min",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "other",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::SpecificTransfer(routine_linkage!(
            |values| todo!(),
            2
        ))),
    },
    dev_src: __static_dev_src!(),
};

pub static I32_MAX: EntityStaticDefn = EntityStaticDefn {
    name: "max",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[StaticParameter {
            name: "other",
            liason: ParameterLiason::Pure,
            ty: "i32",
        }],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::SpecificTransfer(routine_linkage!(
            |values| todo!(),
            2
        ))),
    },
    dev_src: __static_dev_src!(),
};

pub static I32_SGN: EntityStaticDefn = EntityStaticDefn {
    name: "sgn",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::SpecificTransfer(routine_linkage!(
            |values| todo!(),
            1
        ))),
    },
    dev_src: __static_dev_src!(),
};

pub static I32_ABS: EntityStaticDefn = EntityStaticDefn {
    name: "abs",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::SpecificTransfer(routine_linkage!(
            |values| {
                Ok(__TempValue::Copyable(
                    values[0].take_copyable().take_i32().abs().into(),
                ))
            },
            1
        ))),
    },
    dev_src: __static_dev_src!(),
};
