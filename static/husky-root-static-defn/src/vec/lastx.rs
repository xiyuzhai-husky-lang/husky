use super::*;

pub static VEC_LAST: EntityStaticDefn = EntityStaticDefn {
    name: "lastx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::MemberAccess,
        parameters: &[],
        return_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__LinkageGroup::Member(&__MemberLinkageGroup {
            copy_resolved_linkage: resolved_linkage!(virtual_vec_lastx_copy),
            eval_ref_resolved_linkage: resolved_linkage!(virtual_vec_lastx_eval_ref),
            temp_ref_resolved_linkage: resolved_linkage!(virtual_vec_lastx_temp_ref),
            temp_mut_resolved_linkage: resolved_linkage!(virtual_vec_lastx_temp_mut),
            move_resolved_linkage: resolved_linkage!(virtual_vec_lastx_move),
        })),
        output_liason: OutputModifier::MemberAccess {
            member_liason: MemberModifier::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn virtual_vec_lastx_copy<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}

unsafe fn virtual_vec_lastx_eval_ref<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_vec: &'eval DeprecatedVirtualVec =
        values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    virtual_vec.last().unwrap().eval_bind_eval_ref()
}

unsafe fn virtual_vec_lastx_temp_ref<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_vec: &DeprecatedVirtualVec =
        values[0].downcast_temp_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    virtual_vec.last().unwrap().bind_temp_ref()
}

unsafe fn virtual_vec_lastx_temp_mut<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let virtual_vec: &mut DeprecatedVirtualVec =
        values[0].downcast_temp_mut(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    virtual_vec.last_mut().unwrap().bind_temp_mut()
}

fn virtual_vec_lastx_move<'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
}
