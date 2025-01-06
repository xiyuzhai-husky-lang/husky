use super::*;
use dictionary::func_key::VdFuncKeyTranslation;
use either::*;
use lean_mir_expr::{expr::application::LnMirFunc, tactic::LnMirTacticData};
use lean_opr::opr::binary::LnBinaryOpr;
use lean_term::instantiation::LnInstantiation;
use visored_mir_expr::{expr::application::VdMirFunc, hint::VdMirHintIdxRange};
use visored_mir_opr::{opr::binary::VdMirBaseBinaryOpr, separator::VdMirBaseSeparator};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

impl<'a> VdLeanTranspilationBuilder<'a, Sparse> {
    pub(super) fn build_show_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
    ) -> LnItemDefnData {
        todo!()
    }
}
