use super::*;
use crate::{
    block::VdSynBlockIdx, clause::VdSynClauseIdx, division::VdSynDivisionIdx, expr::VdSynExprIdx,
    phrase::VdSynPhraseIdx, sentence::VdSynSentenceIdx,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdSynLineage {
    pub(super) divisions: SmallVec<[VdSynDivisionIdx; 8]>,
    pub(super) stmts: SmallVec<[VdSynBlockIdx; 8]>,
    pub(super) sentence: Option<VdSynSentenceIdx>,
    pub(super) clause: Option<VdSynClauseIdx>,
    pub(super) phrases: SmallVec<[VdSynPhraseIdx; 8]>,
    pub(super) exprs: SmallVec<[VdSynExprIdx; 8]>,
}
impl VdSynLineage {
    pub(crate) fn current_stmt_or_division(
        &self,
    ) -> Either<VdSynBlockIdx, Option<VdSynDivisionIdx>> {
        self.stmts
            .last()
            .copied()
            .map(Left)
            .unwrap_or_else(|| Right(self.divisions.last().copied()))
    }
}
