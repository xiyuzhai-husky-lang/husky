use elaborator::VdBaseqElaboratorInner;
use hypothesis::{contradiction::VdBaseqHypothesisResult, VdBaseqHypothesisIdx};
use visored_mir_expr::{expr::VdMirExprIdx, hint::VdMirHintIdx, stmt::VdMirStmtIdx};

use super::*;

impl<'sess> VdBaseqElaboratorInner<'sess> {
    pub fn obvious(
        &mut self,
        prop: VdMirExprIdx,
    ) -> VdBaseqHypothesisResult<'sess, VdBaseqHypothesisIdx<'sess>> {
        todo!()
    }
}
