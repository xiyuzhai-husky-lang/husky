use super::*;

pub static STD_SLICE_CYCLIC_SLICE_START_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "start",
    items: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        ty: "i32",
        static_linkage_source: &LinkageSource::MemberAccess {
            copy_access: routine_linkage!(cyclic_slice_start_copy_access, 1),
            eval_ref_access: routine_linkage!(|values| todo!(), 1),
            temp_ref_access: routine_linkage!(|values| todo!(), 1),
            temp_mut_access: routine_linkage!(|values| todo!(), 1),
            move_access: routine_linkage!(|values| todo!(), 1),
        },
    },
    dev_src: dev_utils::static_dev_src!(),
};

fn cyclic_slice_start_copy_access<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    Ok(TempValue::Copyable(cyclic_slice.end.into()))
}
