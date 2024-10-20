use crate::{
    clause::VdSemClauseArena, expr::VdSemExprArena, helpers::latex_fmt::VdSemExprLaTeXFormatter,
    phrase::VdSemPhraseArena, sentence::VdSemSentenceArena,
};

pub struct VdSemExprTestBuilder<'db> {
    db: &'db ::salsa::Db,
    expr_arena: VdSemExprArena,
    phrase_arena: VdSemPhraseArena,
    clause_arena: VdSemClauseArena,
    sentence_arena: VdSemSentenceArena,
}

impl<'db> VdSemExprTestBuilder<'db> {
    pub fn latex_formatter<'a>(&'a self) -> VdSemExprLaTeXFormatter<'a> {
        VdSemExprLaTeXFormatter::new(
            self.db,
            self.expr_arena.as_arena_ref(),
            self.phrase_arena.as_arena_ref(),
            self.clause_arena.as_arena_ref(),
            self.sentence_arena.as_arena_ref(),
        )
    }
}
