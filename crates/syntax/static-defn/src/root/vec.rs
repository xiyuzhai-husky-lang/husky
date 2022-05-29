use super::*;
use check_utils::should_eq;

pub static VEC_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Type {
        base_route: "Vec",
        generic_parameters: &[StaticGenericPlaceholder {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        static_trait_impls: &[
            // StaticTraitImplDefn {
            //     dev_src: static_dev_src!(),
            //     trai: "Clone",
            //     member_impls: &[EntityStaticDefn {
            //         dev_src: static_dev_src!(),
            //         name: "clone",
            //         subscopes: &[],
            //         variant: EntityStaticDefnVariant::Method {
            //             this_contract: InputLiason::Pure,
            //             input_parameters: &[],
            //             output_ty: "Vec<E>",
            //             output_liason: OutputLiason::Transfer,
            //             generic_parameters: &[],
            //             kind: MethodStaticDefnVariant::TraitMethodImpl { opt_source: None },
            //         },
            //     }],
            // },
            StaticTraitImplDefn {
                trai: "std::ops::Index<i32>",
                member_impls: &[
                    associated_type_impl!("Output", "E"),
                    EntityStaticDefn {
                        dev_src: static_dev_src!(),
                        name: "index",
                        subscopes: &[],
                        variant: EntityStaticDefnVariant::Method {
                            this_contract: InputLiason::MemberAccess,
                            input_parameters: &[],
                            output_ty: "E",
                            output_liason: OutputLiason::MemberAccess {
                                member_liason: MemberLiason::Mutable,
                            },
                            generic_parameters: &[],
                            kind: MethodStaticDefnVariant::TraitMethodImpl {
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
        visualizer: StaticVisualizer::Vec,
        opt_type_call: Some(&VEC_TYPE_CALL_DEFN),
    },
    dev_src: dev_utils::static_dev_src!(),
};

static VEC_TYPE_CALL_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: vec![],
        output_ty: "Vec<E>",
        output_liason: OutputLiason::Transfer,
        linkage: Linkage {
            call: vec_type_call,
            nargs: 0,
        },
        routine_kind: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
};

fn vec_type_call<'vm, 'eval>(
    _values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    Ok(VMValue::FullyOwned(OwnedValue::new(Vec::<
        MemberValue<'eval>,
    >::new())))
}

fn generic_vec_push<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let element = values[1].into_member();
    let generic_vec: &mut Vec<MemberValue<'eval>> = values[0].downcast_mut();
    generic_vec.push(element);
    Ok(VMValue::Copyable(().into()))
}

fn generic_vec_pop<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

fn generic_vec_first<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

fn generic_vec_last<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

pub(crate) fn construct_generic_vec<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(VMValue::FullyOwned(OwnedValue::new(Vec::<
        MemberValue<'eval>,
    >::new())))
}

pub(crate) fn generic_vec_element_move_access<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

pub(crate) fn generic_vec_element_copy_access<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        VMValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        todo!()
    }
    Ok(this_value[i].copy_into_stack())
}

pub(crate) fn generic_vec_element_ref_access<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        VMValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        return Err(vm_runtime_error!(format!(
            "index out of bounds: the len is {} but the index is {}",
            this_value.len(),
            i
        )));
    }
    let any_ptr: *const (dyn AnyValueDyn<'eval> + 'eval) = this_value[i].any_ref();
    Ok(match values[0] {
        VMValue::GlobalRef(_) => VMValue::GlobalRef(unsafe { &*any_ptr }),
        VMValue::FullyOwnedRef(_) => VMValue::FullyOwnedRef(unsafe { &*any_ptr }),
        _ => panic!(),
    })
}

pub(crate) fn generic_vec_element_borrow_mut_access<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let i: usize = match values[1] {
        VMValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    let (this_value, stack_idx, gen): (&mut Vec<MemberValue<'eval>>, _, _) =
        values[0].downcast_mut_full();
    if i >= this_value.len() {
        todo!()
    }
    Ok(this_value[i].stack_mut(stack_idx))
}

pub static VEC_LEN: EntityStaticDefn = EntityStaticDefn {
    name: "len",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::Pure,
        input_parameters: &[],
        output_ty: "i32",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: generic_list_len,
                nargs: 1,
            }),
        },
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

fn generic_list_len<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let len: i32 = generic_vec.len().try_into().unwrap();
    Ok(VMValue::Copyable(len.into()))
}

pub static VEC_PUSH: EntityStaticDefn = EntityStaticDefn {
    name: "push",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::LocalRefMut,
        input_parameters: &[StaticInputParameter {
            contract: InputLiason::Move,
            ty: "E",
            name: "element",
        }],
        output_ty: "void",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: generic_vec_push,
                nargs: 2,
            }),
        },
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_POP: EntityStaticDefn = EntityStaticDefn {
    name: "pop",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::LocalRefMut,
        input_parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(Linkage {
                call: generic_vec_pop,
                nargs: 1,
            }),
        },
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_FIRST: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::MemberAccess,
        input_parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
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
                    call: generic_vec_first_mut,
                    nargs: 1,
                },
                move_access: Linkage {
                    call: generic_vec_first_move,
                    nargs: 1,
                },
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_first_copy<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

fn generic_vec_first_ref<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    match generic_vec.first() {
        Some(value) => Ok(value.stack_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_mut<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let (generic_vec, stack_idx, gen): (&mut Vec<MemberValue<'eval>>, _, _) =
        values[0].downcast_mut_full();
    match generic_vec.first_mut() {
        Some(value) => Ok(value.stack_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_move<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

pub static VEC_LAST: EntityStaticDefn = EntityStaticDefn {
    name: "last",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: InputLiason::MemberAccess,
        input_parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
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
                    call: generic_vec_last_mut,
                    nargs: 1,
                },
                move_access: Linkage {
                    call: generic_vec_last_move,
                    nargs: 1,
                },
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_last_copy<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}

fn generic_vec_last_ref<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    match generic_vec.last() {
        Some(value) => Ok(value.stack_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_last_mut<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    let (generic_vec, stack_idx, gen): (&mut Vec<MemberValue<'eval>>, _, _) =
        values[0].downcast_mut_full();
    match generic_vec.last_mut() {
        Some(value) => Ok(value.stack_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_last_move<'vm, 'eval>(
    values: &mut [VMValue<'vm, 'eval>],
) -> VMRuntimeResult<VMValue<'vm, 'eval>> {
    todo!()
}
