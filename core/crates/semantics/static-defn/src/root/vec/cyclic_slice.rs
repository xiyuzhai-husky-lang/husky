use print_utils::ps;

use super::*;

pub static VEC_CYCLIC_SLICE: EntityStaticDefn = EntityStaticDefn {
    name: "cyclic_slice",
    subscopes: &[],
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
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::Transfer(linkage!(cyclic_slice, 3)),
        },
        output_liason: OutputLiason::Transfer,
        // bug if output_liason is OutputLiason::MemberAccess
    },
    dev_src: static_dev_src!(),
};

fn cyclic_slice<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let this: &'eval Vec<MemberValue<'eval>> = values[0].downcast_eval_ref();
    let start = values[1].take_copyable().take_i32();
    let end = values[2].take_copyable().take_i32();
    Ok(TempValue::OwnedEval(OwnedValue::new(CyclicSlice::<
        'eval,
        MemberValue<'eval>,
    > {
        start,
        end,
        total: this.as_slice(),
    })))
}
