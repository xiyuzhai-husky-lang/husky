#[salsa::jar]
pub struct VdZfcTypeJar(
    crate::ty::VdZfcType,
    crate::term::VdZfcTermId,
    crate::term::literal::zfc_literal_ty,
    crate::term::zfc_term_to_ty,
    crate::menu::vd_zfc_ty_menu,
    crate::instantiation::VdInstantiation,
    crate::instantiation::menu::vd_zfc_instantiation_menu,
);
