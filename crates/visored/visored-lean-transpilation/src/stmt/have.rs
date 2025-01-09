use super::*;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};
use visored_mir_expr::hypothesis::{
    chunk::VdMirHypothesisChunk, construction::VdMirHypothesisConstruction, VdMirHypothesisIdx,
};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub(super) fn build_have_tactics(
        &mut self,
        stmt: VdMirStmtIdx,
        hypothesis_chunk: VdMirHypothesisChunk,
        ln_tactics: &mut Vec<LnMirTacticData>,
    ) {
        self.build_hypothesis_chunk_tactics(hypothesis_chunk, ln_tactics);
    }
}
