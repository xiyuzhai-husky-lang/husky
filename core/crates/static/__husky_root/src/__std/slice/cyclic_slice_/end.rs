use super::*;

pub static STD_SLICE_CYCLIC_SLICE_END_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "end",
    items: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructOriginal,
        liason: MemberLiason::Mutable,
        ty: "i32",
        linkage: eager_field_linkage!(VirtualCyclicSlice<'eval>, end),
    },
    dev_src: dev_utils::__static_dev_src!(),
};
