#[salsa::db(
    husky_coword::jar::CowordJar,
    visored_zfs_ty::jar::VdZfsTypeJar,
    latex_ast::jar::LxAstJar,
    visored_opr::jar::VdOprJar,
    crate::jar::VdSynExprJar
)]
pub(crate) struct DB {}
