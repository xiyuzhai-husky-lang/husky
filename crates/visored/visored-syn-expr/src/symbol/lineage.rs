use super::*;
use crate::{
    clause::VdSynClauseIdx, division::VdSynDivisionIdx, expr::VdSynExprIdx, phrase::VdSynPhraseIdx,
    sentence::VdSynSentenceIdx, stmt::VdSynStmtIdx,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdSynLineage {
    pub(super) divisions: SmallVec<[VdSynDivisionIdx; 8]>,
    pub(super) stmts: SmallVec<[VdSynStmtIdx; 8]>,
    pub(super) sentence: Option<VdSynSentenceIdx>,
    pub(super) clause: Option<VdSynClauseIdx>,
    pub(super) phrases: SmallVec<[VdSynPhraseIdx; 8]>,
    pub(super) exprs: SmallVec<[VdSynExprIdx; 8]>,
}
