use check_utils::should_eq;

use super::*;

pub static VEC_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "Vec",
        generic_placeholders: &[StaticGenericPlaceholder {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        trait_impls: &[
            StaticTraitImplDefn {
                trai: "Clone",
                member_impls: &[],
            },
            StaticTraitImplDefn {
                trai: "std::ops::Index<i32>",
                member_impls: &[associated_type_impl!("Output", "E")],
            },
        ],
        type_members: &[
            EntityStaticDefn {
                name: "len",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    inputs: &[],
                    output_ty: "i32",
                    generic_placeholders: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::PureOutput(Linkage {
                            call: generic_vec_len,
                            nargs: 0,
                        }),
                    },
                    output_contract: OutputContract::Pure,
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "push",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::BorrowMut,
                    inputs: &[StaticInputPlaceholder {
                        contract: InputContract::Move,
                        ty: "E",
                        name: "element",
                    }],
                    output_ty: "void",
                    generic_placeholders: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::PureOutput(Linkage {
                            call: generic_vec_push,
                            nargs: 0,
                        }),
                    },
                    output_contract: OutputContract::Pure,
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "pop",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::BorrowMut,
                    inputs: &[],
                    output_ty: "E",
                    generic_placeholders: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::PureOutput(Linkage {
                            call: generic_vec_pop,
                            nargs: 0,
                        }),
                    },
                    output_contract: OutputContract::Pure,
                },
                dev_src: static_dev_src!(),
            },
        ],
        variants: &[],
        kind: TyKind::Vec,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: Some(&VEC_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

static VEC_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_placeholders: &[],
        inputs: vec![],
        output_ty: "Vec<E>",
        output_contract: OutputContract::Pure,
        linkage: Linkage {
            call: |values| Ok(StackValue::Boxed(BoxedValue::new(Vec::<VirtualTy>::new()))),
            nargs: 0,
        },
        routine_kind: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_clone<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let generic_vec_cloned: Vec<MemberValue<'eval>> = generic_vec.clone();
    Ok(StackValue::Boxed(BoxedValue::new(generic_vec_cloned)))
}

fn generic_vec_len<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let len: i32 = generic_vec.len().try_into().unwrap();
    Ok(StackValue::Primitive(len.into()))
}

fn generic_vec_push<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 2);
    let element = values[1].into_member();
    let generic_vec: &mut Vec<MemberValue<'eval>> = values[0].downcast_mut();
    generic_vec.push(element);
    Ok(StackValue::Primitive(().into()))
}

fn generic_vec_pop<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_first<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_last<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}
