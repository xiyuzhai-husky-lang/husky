#[salsa::jar]
pub struct VdZfcTypeJar(
    crate::ty::VdZfcType,
    crate::term::VdZfcTermId,
    crate::term::literal::zfc_literal_ty,
    crate::menu::vd_zfc_ty_menu,
);
