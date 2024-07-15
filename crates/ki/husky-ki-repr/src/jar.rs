use crate::*;

#[salsa::jar]
pub struct KiReprJar(
    KiRepr,
    ki_repr_ki,
    crate::repr::val_ki_repr,
    crate::repr::static_var_item_ki_repr,
    crate::var_deps::ki_repr_ki_var_deps,
    KiReprExpansion,
    ki_repr_expansion,
);
