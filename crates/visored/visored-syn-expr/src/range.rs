use latex_ast::ast::{LxAstIdx, LxAstIdxRange};

use crate::{
    clause::VdSynClauseMap, expr::VdSynExprMap, phrase::VdSynPhraseMap, sentence::VdSynSentenceMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynExprAstRange {
    Ast(LxAstIdx),
    Asts(LxAstIdxRange),
}

pub type VdSynPhraseRange = LxAstIdxRange;
pub type VdSynClauseRange = LxAstIdxRange;
pub type VdSynSentenceRange = LxAstIdxRange;

pub type VdSynExprRangeMap = VdSynExprMap<VdSynExprAstRange>;
pub type VdSynPhraseRangeMap = VdSynPhraseMap<VdSynPhraseRange>;
pub type VdSynClauseRangeMap = VdSynClauseMap<VdSynClauseRange>;
pub type VdSynSentenceRangeMap = VdSynSentenceMap<VdSynSentenceRange>;
