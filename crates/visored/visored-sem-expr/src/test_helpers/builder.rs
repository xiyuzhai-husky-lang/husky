use crate::{
    clause::{VdSemClauseArena, VdSemClauseData, VdSemClauseIdx},
    expr::{VdSemExprArena, VdSemExprData, VdSemExprIdx},
    helpers::latex_fmt::VdSemExprLaTeXFormatter,
    phrase::{VdSemPhraseArena, VdSemPhraseData, VdSemPhraseIdx},
    sentence::{VdSemSentenceArena, VdSemSentenceData, VdSemSentenceIdx},
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

impl<'db> VdSemExprTestBuilder<'db> {
    pub fn new_expr_checked(&mut self, data: VdSemExprData, expected: &str) -> VdSemExprIdx {
        let expr = self.expr_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_expr(expr);
        assert_eq!(&formatter.finish(), expected);
        expr
    }

    pub fn new_phrase_checked(&mut self, data: VdSemPhraseData, expected: &str) -> VdSemPhraseIdx {
        let phrase = self.phrase_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_phrase(phrase);
        assert_eq!(&formatter.finish(), expected);
        phrase
    }

    pub fn new_clause_checked(&mut self, data: VdSemClauseData, expected: &str) -> VdSemClauseIdx {
        let clause = self.clause_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_clause(clause);
        assert_eq!(&formatter.finish(), expected);
        clause
    }

    pub fn new_sentence_checked(
        &mut self,
        data: VdSemSentenceData,
        expected: &str,
    ) -> VdSemSentenceIdx {
        let sentence = self.sentence_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_sentence(sentence);
        assert_eq!(&formatter.finish(), expected);
        sentence
    }
}
