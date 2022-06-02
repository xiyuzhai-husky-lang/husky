use super::*;

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
