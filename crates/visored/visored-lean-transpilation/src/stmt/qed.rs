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
        hypothesis: Option<(VdMirHypothesisIdx)>,
    ) -> Vec<LnMirTacticData> {
        todo!()
        // match self.stmt_arena()[stmt].elaboration_tracker().conclusion() {
        //     Some(_) => todo!(),
        //     None => vec![self.default_tactic_data()],
        // }
    }
}
