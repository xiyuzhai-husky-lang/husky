use super::*;
use dictionary::func_key::VdFuncKeyTranslation;
use either::*;
use lean_mir_expr::{expr::application::LnMirFunc, tactic::LnMirTacticData};
use lean_opr::opr::binary::LnBinaryOpr;
use lean_term::instantiation::LnInstantiation;
use visored_mir_expr::{expr::application::VdMirFunc, hint::VdMirHintIdxRange};
use visored_opr::{opr::binary::VdBaseBinaryOpr, separator::VdBaseSeparator};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

impl<'a> VdLeanTranspilationBuilder<'a, Sparse> {
    pub(super) fn build_show_stmt(&mut self, prop: VdMirExprIdx) -> LnItemDefnData {
        todo!()
    }
}
