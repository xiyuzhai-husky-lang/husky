use super::*;

pub static VEC_FIRST: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: linkage!(generic_vec_first_copy, 1),
                eval_ref_access: linkage!(generic_vec_first_eval_ref, 1),
                temp_ref_access: linkage!(generic_vec_first_temp_ref, 1),
                temp_mut_access: linkage!(generic_vec_first_mut, 1),
                move_access: linkage!(generic_vec_first_move, 1),
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_vec_first_copy<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    todo!()
}

fn generic_vec_first_eval_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    match generic_vec.first() {
        Some(value) => Ok(value.bind_eval_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_temp_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let generic_vec: &Vec<MemberValue<'eval>> = values[0].downcast_ref();
    match generic_vec.first() {
        Some(value) => Ok(value.bind_temp_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_mut<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let (generic_vec, stack_idx, gen): (&mut Vec<MemberValue<'eval>>, _, _) =
        values[0].downcast_mut_full();
    match generic_vec.first_mut() {
        Some(value) => Ok(value.bind_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_vec_first_move<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    todo!()
}
