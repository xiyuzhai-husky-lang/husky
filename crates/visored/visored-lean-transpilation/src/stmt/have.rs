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
        let hypothesis_arena = self.hypothesis_arena();
        let construction = hypothesis_arena[hypothesis].construction();
        match construction {
            VdMirHypothesisConstruction::Sorry => self.default_tactics(),
        }
    }
}
