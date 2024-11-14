use crate::{builder::VdHirExprBuilder, ToVdHir};
use visored_opr::opr::binary::VdBaseBinaryOpr;
use visored_sem_expr::expr::{binary::VdSemBinaryDispatch, VdSemExprData};
use visored_zfc_ty::menu::vd_zfc_ty_menu;

#[salsa::db(
    husky_coword::jar::CowordJar,
    latex_ast::jar::LxAstJar,
    latex_command::jar::LxCommandJar,
    latex_environment::jar::LxEnvironmentJar,
    visored_item_path::jar::VdItemPathJar,
    visored_zfc_ty::jar::VdZfcTypeJar,
    visored_opr::jar::VdOprJar,
    visored_sem_expr::jar::VdSemExprJar,
    visored_syn_expr::jar::VdSynExprJar,
    visored_global_dispatch::jar::VdGlobalDispatchJar,
    visored_signature::jar::VdSignatureJar,
    crate::jar::VdMirExprJar
)]
pub(crate) struct DB {}
