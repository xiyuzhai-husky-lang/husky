use super::*;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};
use visored_mir_expr::hypothesis::{construction::VdMirHypothesisConstruction, VdMirHypothesisIdx};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_qed_tactics(
        &mut self,
        stmt: VdMirStmtIdx,
        hypothesis: Option<VdMirHypothesisIdx>,
    ) -> Vec<LnMirTacticData> {
        match hypothesis {
            Some(hypothesis) => self.build_hypothesis_tactics_data(hypothesis),
            None => vec![self.exact_unit()],
        }
    }
}
