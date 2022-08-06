use cyclic_slice::CyclicSlice;

use super::*;

pub static VEC_CYCLIC_SLICE: EntityStaticDefn = EntityStaticDefn {
    name: "cyclic_slice",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::EvalRef,
        parameters: &[
            StaticParameter {
                name: "start",
                liason: ParameterLiason::Pure,
                ty: "i32",
            },
            StaticParameter {
                name: "end",
                liason: ParameterLiason::Pure,
                ty: "i32",
            },
        ],
        return_ty: "[%]E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_cyclic_slice, none)),
        output_liason: OutputLiason::Transfer,
        // bug if output_liason is OutputLiason::MemberAccess
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_cyclic_slice<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let this: &'eval VirtualVec = values[0].downcast_eval_ref(&__VIRTUAL_VEC_VTABLE);
    let start = values[1].downcast_i32();
    let end = values[2].downcast_i32();
    (__Register::new_box(
        VirtualCyclicSlice {
            data: CyclicSlice::<'eval, __Register<'eval>> {
                start,
                end,
                total: this.as_slice(),
            },
        },
        &__VIRTUAL_CYCLIC_SLICE_VTABLE,
    ))
}
