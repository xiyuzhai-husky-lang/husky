use crate::dictionary::func_key::VdFuncKeyTranslation;

use super::*;
use either::*;
use visored_mir_expr::expr::VdMirExprIdxRange;

impl<'db> VdLeanTranspilationBuilder<'db> {
    pub(super) fn build_application(
        &mut self,
        expr: VdMirExprIdx,
        func: VdMirFunc,
        arguments: VdMirExprIdxRange,
    ) -> LnMirExprData {
        match func.key_or_expr(self.db()) {
            Left(func_key) => {
                let Some(translation) = self.dictionary().func_translation(func_key) else {
                    todo!("no translation for func key `{func_key:?}`")
                };
                match translation {
                    VdFuncKeyTranslation::NormalSeparator => todo!(),
                    VdFuncKeyTranslation::InSet => todo!(),
                }
            }
            Right(_) => todo!(),
        }
    }
}
// match function {
//                 VdMirFunc::NormalSeparator => todo!(),
//                 VdMirFunc::InSet => todo!(),
//                 // VdMirApplicationFunction::IntAdd => {
//                 //     debug_assert_eq!(arguments.len(), 2);
//                 //     let lopd = arguments.start();
//                 //     let ropd = lopd + 1;
//                 //     LnMirExprData::Binary {
//                 //         lopd: lopd.to_lean(self),
//                 //         opr: LnBinaryOpr::Add,
//                 //         ropd: ropd.to_lean(self),
//                 //     }
//                 // }
//                 // VdMirApplicationFunction::TrivialEq => {
//                 //     debug_assert_eq!(arguments.len(), 2);
//                 //     let lopd = arguments.start();
//                 //     let ropd = lopd + 1;
//                 //     LnMirExprData::Binary {
//                 //         lopd: lopd.to_lean(self),
//                 //         opr: LnBinaryOpr::Eq,
//                 //         ropd: ropd.to_lean(self),
//                 //     }
//                 // }
//                 // // TODO: implement this
//                 // VdMirApplicationFunction::In => LnMirExprData::Sorry,
//             }
