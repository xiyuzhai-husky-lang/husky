#[salsa::db(
    husky_coword::jar::CowordJar,
    visored_zfc_ty::jar::VdZfcTypeJar,
    visored_opr::jar::VdOprJar,
    latex_ast::jar::LxAstJar,
    latex_command::jar::LxCommandJar,
    latex_environment::jar::LxEnvironmentJar,
    crate::jar::VdSemExprJar,
    visored_syn_expr::jar::VdSynExprJar,
    visored_item_path::jar::VdItemPathJar
)]
pub(crate) struct DB {}
