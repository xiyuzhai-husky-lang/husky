use super::*;
use lean_mir_expr::tactic::LnMirTacticIdxRange;
use visored_mir_expr::hypothesis::{construction::VdMirHypothesisConstruction, VdMirHypothesisIdx};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_have_tactics(
        &mut self,
        stmt: VdMirStmtIdx,
        hypothesis: VdMirHypothesisIdx,
    ) -> LnMirTacticIdxRange {
        self.build_hypothesis_tactics(hypothesis)
    }
}
