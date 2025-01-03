use crate::tactic::VdBaseqTactic;
use alt_option::*;
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
        for tactic in self.session().obvious_tactics() {
            match tactic.run(prop, self)? {
                AltSome(hypothesis_idx) => return Ok(hypothesis_idx),
                AltNone => continue,
            }
        }
        Ok(self
            .hypothesis_constructor
            .construct_new_hypothesis(prop, VdBaseqHypothesisConstruction::Sorry))
    }
}

#[deprecated = "TODO: load tactics from a file"]
pub fn load_obvious_tactics() -> Vec<VdBaseqTactic> {
    vec![VdBaseqTactic::LibrarySearch]
}
