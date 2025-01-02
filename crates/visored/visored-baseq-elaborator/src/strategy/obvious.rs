use elaborator::VdBaseqElaboratorInner;
use expr::VdMirExprFld;
use hypothesis::{
    construction::VdBaseqHypothesisConstruction, contradiction::VdBaseqHypothesisResult,
    VdBaseqHypothesisIdx,
};
use visored_mir_expr::{expr::VdMirExprIdx, hint::VdMirHintIdx, stmt::VdMirStmtIdx};

use super::*;

impl<'db, 'sess> VdBaseqElaboratorInner<'db, 'sess> {
    pub fn obvious(
        &mut self,
        prop: VdMirExprFld<'sess>,
    ) -> VdBaseqHypothesisResult<'sess, VdBaseqHypothesisIdx<'sess>> {
        Ok(self
            .hypothesis_constructor
            .construct_new_hypothesis(prop, VdBaseqHypothesisConstruction::Sorry))
    }
}
