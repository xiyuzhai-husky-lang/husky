use super::*;

pub static STD_SLICE_CYCLIC_SLICE_END_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "end",
    items: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        field_ty: "i32",
        linkage: eager_field_linkage!(
            VirtualCyclicSlice<'eval>,
            __VIRTUAL_CYCLIC_SLICE_VTABLE,
            __I32_VTABLE,
            end,
            direct
        ),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};
