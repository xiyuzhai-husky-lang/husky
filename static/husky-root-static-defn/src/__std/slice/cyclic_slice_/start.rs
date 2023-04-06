use super::*;

pub static STD_SLICE_CYCLIC_SLICE_START_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "start",
    items: &[],
    variant: EntityStaticDefnVariant::TyField {
        field_kind: FieldKind::StructRegular,
        contract: MemberModifier::Mutable,
        field_ty: "i32",
        linkage: eager_field_linkage!(
            mutable,
            Intrinsic,
            Direct,
            DeprecatedVirtualCyclicSlice<'eval>,
            &__DEPRECATED_VIRTUAL_CYCLIC_SLICE_VTABLE,
            i32,
            &__I32_VTABLE,
            start
        ),
    },
    dev_src: husky_dev_utils::static_dev_src!(),
};
