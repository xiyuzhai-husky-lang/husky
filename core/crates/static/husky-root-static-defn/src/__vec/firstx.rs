use super::*;

pub static VEC_FIRST: EntityStaticDefn = EntityStaticDefn {
    name: "firstx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(LinkageDeprecated::Member(&__MemberLinkage {
            copy_access: __SpecificRoutineFp(generic_vec_first_copy),
            eval_ref_access: __SpecificRoutineFp(generic_vec_first_eval_ref),
            temp_ref_access: __SpecificRoutineFp(generic_vec_first_temp_ref),
            temp_mut_access: __SpecificRoutineFp(generic_vec_first_mut),
            move_access: __SpecificRoutineFp(generic_vec_first_move),
            nargs: 1,
            dev_src: __static_dev_src!(),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: __static_dev_src!(),
};

fn generic_vec_first_copy<'temp, 'eval>(
    opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    todo!()
}

fn generic_vec_first_eval_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    generic_vec.first().unwrap().bind_eval_ref()
}

fn generic_vec_first_temp_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let generic_vec: &VirtualVec<'eval> = values[0].downcast_temp_ref();
    generic_vec.first().unwrap().bind_temp_ref()
}

fn generic_vec_first_mut<'temp, 'eval>(
    opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let (generic_vec, stack_idx, gen): (&mut VirtualVec<'eval>, _, _) =
        values[0].downcast_mut_full();
    generic_vec.first_mut().unwrap().bind_mut(stack_idx)
}

fn generic_vec_first_move<'temp, 'eval>(
    opt_ctx: Option<&dyn EvalContextDeprecated<'eval>>,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    todo!()
}
