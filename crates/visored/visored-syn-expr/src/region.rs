use crate::{
    clause::{VdSynClauseArena, VdSynClauseArenaRef},
    expr::{VdSynExprArena, VdSynExprArenaRef, VdSynExprData, VdSynExprIdx},
    phrase::{VdSynPhraseArena, VdSynPhraseArenaRef, VdSynPhraseData, VdSynPhraseIdx},
    sentence::{VdSynSentenceArena, VdSynSentenceArenaRef, VdSynSentenceData, VdSynSentenceIdx},
};

pub struct VdSynExprRegionData {
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
}

impl VdSynExprRegionData {
    pub(crate) fn new(
        expr_arena: VdSynExprArena,
        phrase_arena: VdSynPhraseArena,
        clause_arena: VdSynClauseArena,
        sentence_arena: VdSynSentenceArena,
    ) -> Self {
        Self {
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
        }
    }

    pub fn expr_arena(&self) -> VdSynExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn phrase_arena(&self) -> VdSynPhraseArenaRef {
        self.phrase_arena.as_arena_ref()
    }

    pub fn clause_arena(&self) -> VdSynClauseArenaRef {
        self.clause_arena.as_arena_ref()
    }

    pub fn sentence_arena(&self) -> VdSynSentenceArenaRef {
        self.sentence_arena.as_arena_ref()
    }
}
