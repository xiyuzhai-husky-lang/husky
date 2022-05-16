use check_utils::should_eq;

use super::*;

pub static LIST_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "List",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "List",
        generic_placeholders: &[StaticGenericPlaceholder {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        trait_impls: &[
            StaticTraitImplDefn {
                dev_src: static_dev_src!(),
                trai: "Clone",
                member_impls: &[EntityStaticDefn {
                    dev_src: static_dev_src!(),
                    name: "clone",
                    subscopes: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_contract: InputContract::Pure,
                        input_parameters: &[],
                        output_ty: "List<E>",
                        output_contract: OutputContract::Transfer,
                        generic_parameters: &[],
                        kind: MethodStaticDefnKind::TraitMethodImpl { opt_source: None },
                    },
                }],
            },
            StaticTraitImplDefn {
                trai: "std::ops::Index<i32>",
                member_impls: &[
                    associated_type_impl!("Output", "E"),
                    EntityStaticDefn {
                        dev_src: static_dev_src!(),
                        name: "index",
                        subscopes: &[],
                        variant: EntityStaticDefnVariant::Method {
                            this_contract: InputContract::MemberAccess,
                            input_parameters: &[],
                            output_ty: "E",
                            output_contract: OutputContract::MemberAccess,
                            generic_parameters: &[],
                            kind: MethodStaticDefnKind::TraitMethodImpl {
                                opt_source: Some(LinkageSource::MemberAccess {
                                    copy_access: Linkage {
                                        call: generic_vec_element_copy_access,
                                        nargs: 2,
                                    },
                                    ref_access: Linkage {
                                        call: generic_vec_element_ref_access,
                                        nargs: 2,
                                    },
                                    move_access: Linkage {
                                        call: generic_vec_element_move_access,
                                        nargs: 2,
                                    },
                                    ref_mut_access: Linkage {
                                        call: generic_vec_element_borrow_mut_access,
                                        nargs: 2,
                                    },
                                }),
                            },
                        },
                    },
                ],
                dev_src: static_dev_src!(),
            },
        ],
        type_members: &[
            EntityStaticDefn {
                name: "len",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::Pure,
                    input_parameters: &[],
                    output_ty: "i32",
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: generic_list_len,
                            nargs: 1,
                        }),
                    },
                    output_contract: OutputContract::Transfer,
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "push",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::BorrowMut,
                    input_parameters: &[StaticInputParameter {
                        contract: InputContract::Move,
                        ty: "E",
                        name: "element",
                    }],
                    output_ty: "void",
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: generic_vec_push,
                            nargs: 2,
                        }),
                    },
                    output_contract: OutputContract::Transfer,
                },
                dev_src: static_dev_src!(),
            },
            EntityStaticDefn {
                name: "pop",
                subscopes: &[],
                variant: EntityStaticDefnVariant::Method {
                    this_contract: InputContract::BorrowMut,
                    input_parameters: &[],
                    output_ty: "E",
                    generic_parameters: &[],
                    kind: MethodStaticDefnKind::TypeMethod {
                        source: LinkageSource::Transfer(Linkage {
                            call: generic_vec_pop,
                            nargs: 1,
                        }),
                    },
                    output_contract: OutputContract::Transfer,
                },
                dev_src: static_dev_src!(),
            },
        ],
        variants: &[],
        kind: TyKind::Vec,
        visualizer: TRIVIAL_VISUALIZER,
        opt_type_call: Some(&LIST_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

static LIST_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "List",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_placeholders: &[],
        input_placeholders: vec![],
        output_ty: "List<E>",
        output_contract: OutputContract::Transfer,
        linkage: Linkage {
            call: vec_type_call,
            nargs: 0,
        },
        routine_kind: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
};

fn vec_type_call<'stack, 'eval>(
    _values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<MemberValue<'eval>>::new(),
    )))
}

// fn generic_vec_clone<'stack, 'eval>(
//     values: &mut [StackValue<'stack, 'eval>],
// ) -> VMResult<StackValue<'stack, 'eval>> {
//     let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
//     let generic_vec_cloned: Vec<MemberValue<'eval>> = generic_vec.clone();
//     Ok(StackValue::Boxed(BoxedValue::new(generic_vec_cloned)))
// }

fn generic_list_len<'stack, 'eval>(
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

pub(crate) fn construct_generic_vec<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<MemberValue<'eval>>::new(),
    )))
}

pub(crate) fn generic_vec_element_move_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

pub(crate) fn generic_vec_element_copy_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        StackValue::Primitive(value) => value.as_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i > this_value.len() {
        todo!()
    }
    Ok(this_value[i].copy_into_stack())
}

pub(crate) fn generic_vec_element_ref_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        StackValue::Primitive(value) => value.as_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i > this_value.len() {
        todo!()
    }
    Ok(this_value[i].stack_ref())
}

pub(crate) fn generic_vec_element_borrow_mut_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let i: usize = match values[1] {
        StackValue::Primitive(value) => value.as_i32().try_into().unwrap(),
        _ => panic!(),
    };
    let (this_value, stack_idx, gen): (&mut Vec<MemberValue<'eval>>, _, _) =
        values[0].downcast_mut_full();
    if i > this_value.len() {
        todo!()
    }
    Ok(this_value[i].stack_mut(stack_idx))
}
