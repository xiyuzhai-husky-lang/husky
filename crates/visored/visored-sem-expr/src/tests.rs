#[salsa::db(
    husky_coword::jar::CowordJar,
    visored_term::jar::VdTermJar,
    visored_opr::jar::VdOprJar,
    latex_ast::jar::LxAstJar,
    latex_command::jar::LxCommandJar,
    latex_environment::jar::LxEnvironmentJar,
    crate::jar::VdSemExprJar,
    visored_syn_expr::jar::VdSynExprJar,
    visored_item_path::jar::VdItemPathJar,
    visored_global_dispatch::jar::VdGlobalDispatchJar,
    visored_signature::jar::VdSignatureJar,
    visored_vfs::jar::VdVfsJar
)]
pub(crate) struct DB {}
