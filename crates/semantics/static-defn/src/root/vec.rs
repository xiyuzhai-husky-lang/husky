mod cyclic_slice;
mod first;
mod last;

pub use cyclic_slice::*;
pub use first::*;
pub use last::*;

use super::*;
use check_utils::should_eq;

pub static VEC_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "Vec",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "Vec",
        generic_parameters: &[StaticGenericPlaceholder {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        static_trait_impls: &[StaticTraitImplDefn {
            trai: "std::ops::Index<i32>",
            member_impls: &[
                associated_type_impl!("Output", "E"),
                EntityStaticDefn {
                    dev_src: static_dev_src!(),
                    name: "index",
                    subscopes: &[],
                    variant: EntityStaticDefnVariant::Method {
                        this_contract: ParameterLiason::MemberAccess,
                        parameters: &[],
                        output_ty: "E",
                        output_liason: OutputLiason::MemberAccess {
                            member_liason: MemberLiason::Mutable,
                        },
                        generic_parameters: &[],
                        kind: MethodStaticDefnVariant::TraitMethodImpl {
                            opt_source: Some(LinkageSource::MemberAccess {
                                copy_access: linkage!(generic_vec_element_copy_access, 2),
                                ref_access: linkage!(generic_vec_element_ref_access, 2),
                                move_access: linkage!(generic_vec_element_move_access, 2),
                                ref_mut_access: linkage!(generic_vec_element_borrow_mut_access, 2),
                            }),
                        },
                    },
                },
            ],
            dev_src: static_dev_src!(),
        }],
        ty_members: &[
            &VEC_LEN,
            &VEC_PUSH,
            &VEC_POP,
            &VEC_FIRST,
            &VEC_LAST,
            &VEC_CYCLIC_SLICE,
        ],
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
        linkage: linkage!(vec_type_call, 0),
        routine_kind: RoutineKind::TypeCall,
    },
    dev_src: static_dev_src!(),
};

fn vec_type_call<'vm, 'eval>(
    _values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    Ok(TempValue::EvalOwned(OwnedValue::new(Vec::<
        MemberValue<'eval>,
    >::new())))
}

fn generic_vec_push<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let element = values[1].into_member();
    let generic_vec: &mut Vec<MemberValue<'eval>> = values[0].downcast_mut();
    generic_vec.push(element);
    Ok(TempValue::Copyable(().into()))
}

fn generic_vec_pop<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    todo!()
}

pub(crate) fn construct_generic_vec<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(TempValue::EvalOwned(OwnedValue::new(Vec::<
        MemberValue<'eval>,
    >::new())))
}

pub(crate) fn generic_vec_element_move_access<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    todo!()
}

pub(crate) fn generic_vec_element_copy_access<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
        _ => panic!(),
    };
    if i >= this_value.len() {
        todo!()
    }
    Ok(this_value[i].copy_into_stack())
}

pub(crate) fn generic_vec_element_ref_access<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let this_value: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
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
        TempValue::EvalRef(_) => TempValue::EvalRef(unsafe { &*any_ptr }),
        TempValue::FullyOwnedRef(_) => TempValue::FullyOwnedRef(unsafe { &*any_ptr }),
        _ => panic!(),
    })
}

pub(crate) fn generic_vec_element_borrow_mut_access<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let i: usize = match values[1] {
        TempValue::Copyable(value) => value.take_i32().try_into().unwrap(),
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
        this_contract: ParameterLiason::Pure,
        parameters: &[],
        output_ty: "i32",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(generic_vec_len, 1)),
        },
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_len<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    let len: i32 = generic_vec.len().try_into().unwrap();
    Ok(TempValue::Copyable(len.into()))
}

pub static VEC_PUSH: EntityStaticDefn = EntityStaticDefn {
    name: "push",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: ParameterLiason::TempRefMut,
        parameters: &[StaticParameter {
            contract: ParameterLiason::Move,
            ty: "E",
            name: "element",
        }],
        output_ty: "void",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(generic_vec_push, 2)),
        },
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};

pub static VEC_POP: EntityStaticDefn = EntityStaticDefn {
    name: "pop",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: ParameterLiason::TempRefMut,
        parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(generic_vec_pop, 1)),
        },
        output_liason: OutputLiason::Transfer,
    },
    dev_src: static_dev_src!(),
};
