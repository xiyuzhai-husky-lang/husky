use super::*;
use check_utils::should_eq;

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
                dev_src: static_dev_src!(),
                trai: "Clone",
                member_impls: &[EntityStaticDefn {
                    dev_src: static_dev_src!(),
                    name: "clone",
                    subscopes: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_contract: InputContract::Pure,
                        input_parameters: &[],
                        output_ty: "Vec<E>",
                        output_contract: OutputLiason::Transfer,
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
                            output_contract: OutputLiason::MemberAccess,
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
        type_members: &[&VEC_LEN, &VEC_PUSH, &VEC_POP, &VEC_FIRST, &VEC_LAST],
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
        input_placeholders: vec![],
        output_ty: "Vec<E>",
        output_contract: OutputLiason::Transfer,
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
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<MemberValue<'eval>>::new(),
    )))
}

fn generic_vec_push<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 2);
    let element = values[1].into_member();
    let generic_vec: &mut Vec<MemberValue<'eval>> = values[0].downcast_mut();
    generic_vec.push(element);
    Ok(StackValue::Primitive(().into()))
}

fn generic_vec_pop<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_first<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_last<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

pub(crate) fn construct_generic_vec<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<MemberValue<'eval>>::new(),
    )))
}

pub(crate) fn generic_vec_element_move_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

pub(crate) fn generic_vec_element_copy_access<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
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
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
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
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
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

pub static VEC_LEN: EntityStaticDefn = EntityStaticDefn {
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
        output_contract: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

fn generic_list_len<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let len: i32 = generic_vec.len().try_into().unwrap();
    Ok(StackValue::Primitive(len.into()))
}

pub static VEC_PUSH: EntityStaticDefn = EntityStaticDefn {
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
        output_contract: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_POP: EntityStaticDefn = EntityStaticDefn {
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
        output_contract: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_FIRST: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputContract::MemberAccess,
        input_parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnKind::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: Linkage {
                    call: generic_vec_first_copy,
                    nargs: 1,
                },
                ref_access: Linkage {
                    call: generic_vec_first_ref,
                    nargs: 1,
                },
                ref_mut_access: Linkage {
                    call: generic_vec_first_ref_mut,
                    nargs: 1,
                },
                move_access: Linkage {
                    call: generic_vec_first_move,
                    nargs: 1,
                },
            },
        },
        output_contract: OutputLiason::MemberAccess,
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_first_copy<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_first_ref<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_first_ref_mut<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_first_move<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

pub static VEC_LAST: EntityStaticDefn = EntityStaticDefn {
    name: "last",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputContract::MemberAccess,
        input_parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnKind::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: Linkage {
                    call: generic_vec_last_copy,
                    nargs: 1,
                },
                ref_access: Linkage {
                    call: generic_vec_last_ref,
                    nargs: 1,
                },
                ref_mut_access: Linkage {
                    call: generic_vec_last_ref_mut,
                    nargs: 1,
                },
                move_access: Linkage {
                    call: generic_vec_last_move,
                    nargs: 1,
                },
            },
        },
        output_contract: OutputLiason::MemberAccess,
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_last_copy<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_last_ref<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_last_ref_mut<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn generic_vec_last_move<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMRuntimeResult<StackValue<'stack, 'eval>> {
    todo!()
}
