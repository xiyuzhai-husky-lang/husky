use super::*;

pub static B32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "b32",
        generic_placeholders: &[],
        trait_impls: &[],
        type_members: &[&B32_TRAILING_ZEROS],
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

pub static B32_TRAILING_ZEROS: EntityStaticDefn = EntityStaticDefn {
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
};
