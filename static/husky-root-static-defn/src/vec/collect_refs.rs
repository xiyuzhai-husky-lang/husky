use super::*;

pub static VEC_COLLECT_REFS: EntityStaticDefn = EntityStaticDefn {
    name: "collect_refs",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::Leash,
        parameters: &[],
        return_ty: "[]&E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_vec_collect_refs, none)),
        output_liason: OutputModifier::Transfer,
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_vec_collect_refs<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    todo!()
    // let this: &'eval DeprecatedVirtualVec = values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    // let start = values[1].downcast_i32();
    // let end = values[2].downcast_i32();
    // (__Register::new_box(
    //     DeprecatedVirtualCyclicSlice {
    //         data: CyclicSlice::<'eval, __Register<'eval>> {
    //             start,
    //             end,
    //             total: this.as_slice(),
    //         },
    //     },
    //     &__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE,
    // ))
}
