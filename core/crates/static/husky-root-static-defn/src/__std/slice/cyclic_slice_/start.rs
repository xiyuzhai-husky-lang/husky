use super::*;

pub static STD_SLICE_CYCLIC_SLICE_START_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "start",
    items: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        ty: "i32",
        linkage: eager_field_linkage!(VirtualCyclicSlice<'eval>, &__I32_VTABLE, start, direct),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};
