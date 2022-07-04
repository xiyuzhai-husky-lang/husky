use super::*;

pub static VEC_LAST: EntityStaticDefn = EntityStaticDefn {
    name: "lastx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_access: __SpecificRoutineFp(generic_vec_lastx_copy),
            eval_ref_access: __SpecificRoutineFp(generic_vec_lastx_eval_ref),
            temp_ref_access: __SpecificRoutineFp(generic_vec_lastx_temp_ref),
            temp_mut_access: __SpecificRoutineFp(generic_vec_lastx_mut),
            move_access: __SpecificRoutineFp(generic_vec_lastx_move),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: __static_dev_src!(),
};

fn generic_vec_lastx_copy<'temp, 'eval>(
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    todo!()
}

fn generic_vec_lastx_eval_ref<'temp, 'eval>(
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    generic_vec.last().unwrap().bind_eval_ref()
}

fn generic_vec_lastx_temp_ref<'temp, 'eval>(
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    generic_vec.last().unwrap().bind_temp_ref()
}

fn generic_vec_lastx_mut<'temp, 'eval>(
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let (generic_vec, stack_idx, gen): (&mut VirtualVec<'eval>, _, _) =
        values[0].downcast_mut_full();
    generic_vec.last_mut().unwrap().bind_mut(stack_idx)
}

fn generic_vec_lastx_move<'temp, 'eval>(
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    todo!()
}
