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
        output_ty: "[%]E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::GenericTransfer(generic_routine_linkage!(
            generic_cyclic_slice,
            3
        ))),
        output_liason: OutputLiason::Transfer,
        // bug if output_liason is OutputLiason::MemberAccess
    },
    dev_src: __static_dev_src!(),
};

fn generic_cyclic_slice<'temp, 'eval>(
    ty: EntityRoutePtr,
    values: &mut [__TempValue<'temp, 'eval>],
) -> __TempValue<'temp, 'eval> {
    let this: &'eval VirtualVec<'eval> = values[0].downcast_eval_ref();
    let start = values[1].take_copyable().take_i32();
    let end = values[2].take_copyable().take_i32();
    __TempValue::OwnedEval(__OwnedValue::new(VirtualCyclicSlice {
        data: CyclicSlice::<'eval, MemberValue<'eval>> {
            start,
            end,
            total: this.as_slice(),
        },
        ty,
    }))
}
