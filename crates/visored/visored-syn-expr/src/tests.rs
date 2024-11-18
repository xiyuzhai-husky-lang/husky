#[salsa::db(
    husky_coword::jar::CowordJar,
    visored_term::jar::VdTermJar,
    latex_ast::jar::LxAstJar,
    latex_command::jar::LxCommandJar,
    latex_environment::jar::LxEnvironmentJar,
    visored_opr::jar::VdOprJar,
    visored_item_path::jar::VdItemPathJar,
    visored_vfs::jar::VdVfsJar,
    crate::jar::VdSynExprJar
)]
pub(crate) struct DB {}
