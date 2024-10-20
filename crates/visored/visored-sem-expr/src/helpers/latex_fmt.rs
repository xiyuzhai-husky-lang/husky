use crate::{
    clause::VdSemClauseArenaRef,
    expr::VdSemExprArenaRef,
    phrase::VdSemPhraseArenaRef,
    sentence::{VdSemSentenceArenaRef, VdSemSentenceIdxRange},
};

pub struct VdSemExprLaTeXFormatter<'a> {
    db: &'a ::salsa::Db,
    expr_arena: VdSemExprArenaRef<'a>,
    phrase_arena: VdSemPhraseArenaRef<'a>,
    clause_arena: VdSemClauseArenaRef<'a>,
    sentence_arena: VdSemSentenceArenaRef<'a>,
    result: String,
}

impl<'a> VdSemExprLaTeXFormatter<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        expr_arena: VdSemExprArenaRef<'a>,
        phrase_arena: VdSemPhraseArenaRef<'a>,
        clause_arena: VdSemClauseArenaRef<'a>,
        sentence_arena: VdSemSentenceArenaRef<'a>,
    ) -> Self {
        Self {
            db,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            result: Default::default(),
        }
    }

    pub fn fmt_sentences(&mut self, sentences: VdSemSentenceIdxRange) {
        todo!()
    }
}
