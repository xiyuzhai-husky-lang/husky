use super::*;

pub static VEC_FIRST: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: ParameterLiason::MemberAccess,
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
