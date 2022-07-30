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
            copy_fp: linkage_fp!(virtual_vec_lastx_copy),
            eval_ref_fp: linkage_fp!(virtual_vec_lastx_eval_ref),
            temp_ref_fp: linkage_fp!(virtual_vec_lastx_temp_ref),
            temp_mut_fp: linkage_fp!(virtual_vec_lastx_temp_mut),
            move_fp: linkage_fp!(virtual_vec_lastx_move),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn virtual_vec_lastx_copy<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

unsafe fn virtual_vec_lastx_eval_ref<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_vec: &'eval VirtualVec = values[0].downcast_eval_ref();
    virtual_vec.last().unwrap().eval_bind_eval_ref()
}

unsafe fn virtual_vec_lastx_temp_ref<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_vec: &VirtualVec = values[0].downcast_temp_ref();
    virtual_vec.last().unwrap().bind_temp_ref()
}

unsafe fn virtual_vec_lastx_temp_mut<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let virtual_vec: &mut VirtualVec = values[0].downcast_temp_mut();
    virtual_vec.last_mut().unwrap().bind_temp_mut()
}

fn virtual_vec_lastx_move<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}
