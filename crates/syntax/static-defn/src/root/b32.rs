use super::*;

pub static B32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b32",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "b32",
        generic_placeholders: &[],
        static_trait_impls: &[],
        type_members: &[&B32_TRAILING_ZEROS, &B32_LAST_BITS],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer::Compiled(|value| {
            let value: &u32 = value.downcast_ref();
            VisualProps::Primitive {
                value: (*value).into(),
            }
        }),
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B32_TRAILING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "trailing_zeros",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "i32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| {
                    Ok(StackValue::Copyable(
                        (values[0].take_copyable().take_b32().trailing_zeros() as i32).into(),
                    ))
                },
                nargs: 1,
            }),
        },
    },
    dev_src: static_dev_src!(),
};

pub static B32_LAST_BITS: EntityStaticDefn = EntityStaticDefn {
    name: "last_bits",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[StaticInputParameter {
            name: "k",
            contract: InputLiason::Pure,
            ty: "i32",
        }],
        output_ty: "b32",
        output_liason: OutputLiason::Transfer,
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: |values| {
                    let b = values[0].take_copyable().take_b32();
                    let i = values[1].take_copyable().take_i32();
                    let last_bits = b & ((1 << i) - 1);
                    Ok(StackValue::Copyable(last_bits.into()))
                },
                nargs: 2,
            }),
        },
    },
    dev_src: static_dev_src!(),
};
