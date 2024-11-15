#[salsa::jar]
pub struct VdZfcTypeJar(
    crate::ty::VdZfcType,
    crate::ty::is_vd_zfc_ty_function_like,
    crate::term::VdZfcTermId,
    crate::term::literal::zfc_literal_ty,
    crate::term::zfc_term_to_ty,
    crate::term::menu::vd_zfc_term_menu,
    crate::menu::vd_zfc_ty_menu,
    crate::instantiation::VdInstantiation,
    crate::instantiation::menu::vd_zfc_instantiation_menu,
);
