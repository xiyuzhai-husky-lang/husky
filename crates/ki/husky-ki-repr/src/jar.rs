use crate::*;

#[salsa::jar]
pub struct KiReprJar(
    KiRepr,
    ki_repr_ki,
    val_item_ki_repr,
    KiReprExpansion,
    ki_repr_expansion,
);
