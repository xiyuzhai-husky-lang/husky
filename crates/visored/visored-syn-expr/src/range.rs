use latex_ast::ast::{LxAstIdx, LxAstIdxRange};

use crate::{
    clause::VdSynClauseOrderedMap, expr::VdSynExprOrderedMap, phrase::VdSynPhraseOrderedMap,
    sentence::VdSynSentenceOrderedMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynExprAstRange {
    Ast(LxAstIdx),
    Asts(LxAstIdxRange),
}

pub type VdSynPhraseRange = LxAstIdxRange;
pub type VdSynClauseRange = LxAstIdxRange;
pub type VdSynSentenceRange = LxAstIdxRange;

pub type VdSynExprRangeMap = VdSynExprOrderedMap<VdSynExprAstRange>;
pub type VdSynPhraseRangeMap = VdSynPhraseOrderedMap<VdSynPhraseRange>;
pub type VdSynClauseRangeMap = VdSynClauseOrderedMap<VdSynClauseRange>;
pub type VdSynSentenceRangeMap = VdSynSentenceOrderedMap<VdSynSentenceRange>;
