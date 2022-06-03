use super::*;

pub static VEC_CYCLIC_SLICE: EntityStaticDefn = EntityStaticDefn {
    name: "cyclic_slice",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_contract: ParameterLiason::EvalRef,
        parameters: &[
            StaticParameter {
                name: "start",
                contract: ParameterLiason::Pure,
                ty: "i32",
            },
            StaticParameter {
                name: "end",
                contract: ParameterLiason::Pure,
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

fn cyclic_slice<'vm, 'eval>(
    values: &mut [TempValue<'vm, 'eval>],
) -> VMRuntimeResult<TempValue<'vm, 'eval>> {
    let this: &'eval Vec<MemberValue<'eval>> = values[0].downcast_eval_ref();
    let start = values[1].take_copyable().take_i32();
    let end = values[2].take_copyable().take_i32();
    Ok(TempValue::EvalOwned(OwnedValue::new(CyclicSlice::<
        'eval,
        MemberValue<'eval>,
    > {
        start,
        end,
        total: this.as_slice(),
    })))
}
