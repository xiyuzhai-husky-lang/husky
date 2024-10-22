use crate::{
    clause::{VdSemClauseArena, VdSemClauseArenaRef, VdSemClauseData, VdSemClauseIdx},
    expr::{VdSemExprArena, VdSemExprArenaRef, VdSemExprData, VdSemExprIdx},
    helpers::latex_fmt::VdSemExprLaTeXFormatter,
    phrase::{VdSemPhraseArena, VdSemPhraseArenaRef, VdSemPhraseData, VdSemPhraseIdx},
    sentence::{VdSemSentenceArena, VdSemSentenceArenaRef, VdSemSentenceData, VdSemSentenceIdx},
};

pub struct VdSemExprRegionData {
    expr_arena: VdSemExprArena,
    phrase_arena: VdSemPhraseArena,
    clause_arena: VdSemClauseArena,
    sentence_arena: VdSemSentenceArena,
}

impl VdSemExprRegionData {
    pub(crate) fn new(
        expr_arena: VdSemExprArena,
        phrase_arena: VdSemPhraseArena,
        clause_arena: VdSemClauseArena,
        sentence_arena: VdSemSentenceArena,
    ) -> Self {
        Self {
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
        }
    }

    pub fn expr_arena(&self) -> VdSemExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn phrase_arena(&self) -> VdSemPhraseArenaRef {
        self.phrase_arena.as_arena_ref()
    }

    pub fn clause_arena(&self) -> VdSemClauseArenaRef {
        self.clause_arena.as_arena_ref()
    }

    pub fn sentence_arena(&self) -> VdSemSentenceArenaRef {
        self.sentence_arena.as_arena_ref()
    }
}
