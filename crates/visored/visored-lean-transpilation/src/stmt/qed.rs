use super::*;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};
use visored_mir_expr::hypothesis::{
    chunk::VdMirHypothesisChunk, construction::VdMirHypothesisConstruction, VdMirHypothesisIdx,
};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_qed_tactics(
        &mut self,
        stmt: VdMirStmtIdx,
        hypothesis_chunk: Option<VdMirHypothesisChunk>,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        match hypothesis_chunk {
            Some(hypothesis_chunk) => {
                // ad hoc
                // self.build_hypothesis_chunk_tactics(hypothesis_chunk, ln_tactics)
                ln_tactics.push(self.default_tactic_data());
            }
            None => {
                ln_tactics.push(self.exact_unit());
            }
        }
    }
}
