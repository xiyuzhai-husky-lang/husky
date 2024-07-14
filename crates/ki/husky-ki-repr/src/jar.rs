use crate::*;

#[salsa::jar]
pub struct KiReprJar(
    KiRepr,
    ki_repr_ki,
    crate::repr::val_item_ki_repr,
    crate::repr::static_var_item_ki_repr,
    crate::static_var_deps::ki_ki_static_var_deps,
    KiReprExpansion,
    ki_repr_expansion,
);
