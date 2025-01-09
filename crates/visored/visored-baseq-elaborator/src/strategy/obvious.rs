use crate::tactic::VdBsqTactic;
use alt_option::*;
use elaborator::VdBsqElaboratorInner;
use expr::VdBsqExprFld;
use hypothesis::{
    construction::VdBsqHypothesisConstruction, contradiction::VdBsqHypothesisResult,
    VdBsqHypothesisIdx,
};
use visored_mir_expr::{expr::VdMirExprIdx, hint::VdMirHintIdx, stmt::VdMirStmtIdx};

use super::*;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn obvious(
        &mut self,
        prop: VdBsqExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        self.with_call(VdBsqStrategyCall::Obvious, |slf| slf.obvious_inner(prop))
    }

    fn obvious_inner(
        &mut self,
        prop: VdBsqExprFld<'sess>,
    ) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        for tactic in self.session().obvious_tactics() {
            match tactic.run(prop, self)? {
                AltSome(hypothesis_idx) => return Ok(hypothesis_idx),
                AltNone => continue,
            }
        }
        Ok(self
            .hypothesis_constructor
            .construct_new_hypothesis(prop, VdBsqHypothesisConstruction::Sorry))
    }
}

#[deprecated = "TODO: load tactics from a file"]
pub fn load_obvious_tactics() -> Vec<VdBsqTactic> {
    vec![
        // order matters!!!
        // assumption should always be the first tactic
        VdBsqTactic::Assumption,
        VdBsqTactic::TermTrivial,
        VdBsqTactic::LibrarySearch,
        VdBsqTactic::CommRing,
    ]
}
