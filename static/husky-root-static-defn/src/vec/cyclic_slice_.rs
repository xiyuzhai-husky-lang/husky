use cyclic_slice::CyclicSlice;

use super::*;

pub static VEC_CYCLIC_SLICE: EntityStaticDefn = EntityStaticDefn {
    name: "cyclic_slice",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::Leash,
        parameters: &[
            StaticParameter {
                name: "start",
                modifier: ParameterModifier::None,
                ty: "i32",
            },
            StaticParameter {
                name: "end",
                modifier: ParameterModifier::None,
                ty: "i32",
            },
        ],
        return_ty: "[%]E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(virtual_vec_cyclic_slice, none)),
        output_liason: OutputModifier::Transfer,
        // bug if output_liason is OutputModifier::MemberAccess
    },
    dev_src: static_dev_src!(),
};

unsafe fn virtual_vec_cyclic_slice<'temp, 'eval>(
    values: &mut [__Register<'eval>],
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
) -> __Register<'eval> {
    let this: &'eval DeprecatedVirtualVec =
        values[0].downcast_eval_ref(&__DEPRECATED_VIRTUAL_VEC_VTABLE);
    let start = values[1].downcast_i32();
    let end = values[2].downcast_i32();
    __Register::new_box(
        DeprecatedVirtualCyclicSlice {
            data: CyclicSlice::<'eval, __Register<'eval>> {
                start,
                end,
                total: this.as_slice(),
            },
        },
        &__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE,
    )
}
