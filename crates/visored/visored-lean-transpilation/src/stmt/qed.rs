use super::*;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};
use visored_mir_expr::hypothesis::VdMirHypothesisIdx;

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
            Some(_) => todo!(),
            None => vec![self.exact_unit()],
        }
    }
}
