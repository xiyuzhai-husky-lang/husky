use crate::{
    clause::VdCnlClauseArena, decompose::Decompose, expr::VdCnlExprArena, phrase::VdCnlPhraseArena,
    sentence::VdCnlSentenceArena,
};
use is_llm::IsLlm;

pub struct VdCnlParser<'db> {
    llm: &'db dyn IsLlm,
    expr_arena: VdCnlExprArena,
    phrase_arena: VdCnlPhraseArena,
    clause_arena: VdCnlClauseArena,
    sentence_arena: VdCnlSentenceArena,
}

impl<'db> VdCnlParser<'db> {
    pub fn new(llm: &'db dyn IsLlm) -> Self {
        Self {
            llm,
            expr_arena: Default::default(),
            phrase_arena: Default::default(),
            clause_arena: Default::default(),
            sentence_arena: Default::default(),
        }
    }
}

impl<'db> VdCnlParser<'db> {}

impl<'db> VdCnlParser<'db> {
    pub fn expr_arena_mut(&mut self) -> &mut VdCnlExprArena {
        &mut self.expr_arena
    }

    pub fn phrase_arena_mut(&mut self) -> &mut VdCnlPhraseArena {
        &mut self.phrase_arena
    }

    pub fn clause_arena_mut(&mut self) -> &mut VdCnlClauseArena {
        &mut self.clause_arena
    }

    pub fn sentence_arena_mut(&mut self) -> &mut VdCnlSentenceArena {
        &mut self.sentence_arena
    }
}

impl<'db> VdCnlParser<'db> {
    pub(crate) fn decompose<D: Decompose>(&mut self) {
        todo!()
    }
}
