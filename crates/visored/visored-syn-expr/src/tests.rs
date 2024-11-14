#[salsa::db(
    husky_coword::jar::CowordJar,
    visored_zfc_ty::jar::VdZfcTypeJar,
    latex_ast::jar::LxAstJar,
    latex_command::jar::LxCommandJar,
    latex_environment::jar::LxEnvironmentJar,
    visored_opr::jar::VdOprJar,
    visored_item_path::jar::VdItemPathJar,
    crate::jar::VdSynExprJar
)]
pub(crate) struct DB {}
