use crate::{
    clause::{VdSynClauseArena, VdSynClauseData, VdSynClauseIdx},
    expr::{VdSynExprArena, VdSynExprData, VdSynExprIdx},
    helpers::latex_fmt::VdSynExprLaTeXFormatter,
    phrase::{VdSynPhraseArena, VdSynPhraseData, VdSynPhraseIdx},
    region::VdSynExprRegionData,
    sentence::{VdSynSentenceArena, VdSynSentenceData, VdSynSentenceIdx},
};

pub struct VdSynExprTestBuilder<'db> {
    db: &'db ::salsa::Db,
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
}

impl<'db> VdSynExprTestBuilder<'db> {
    pub fn new(db: &'db ::salsa::Db) -> Self {
        Self {
            db,
            expr_arena: VdSynExprArena::default(),
            phrase_arena: VdSynPhraseArena::default(),
            clause_arena: VdSynClauseArena::default(),
            sentence_arena: VdSynSentenceArena::default(),
        }
    }
}

impl<'db> VdSynExprTestBuilder<'db> {
    pub fn latex_formatter<'a>(&'a self) -> VdSynExprLaTeXFormatter<'a> {
        VdSynExprLaTeXFormatter::new(
            self.db,
            self.expr_arena.as_arena_ref(),
            self.phrase_arena.as_arena_ref(),
            self.clause_arena.as_arena_ref(),
            self.sentence_arena.as_arena_ref(),
        )
    }
}

impl<'db> VdSynExprTestBuilder<'db> {
    #[track_caller]
    pub fn new_expr_checked(&mut self, data: VdSynExprData, expected: &str) -> VdSynExprIdx {
        let expr = self.expr_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_expr(expr);
        assert_eq!(&formatter.finish(), expected);
        expr
    }

    #[track_caller]
    pub fn new_phrase_checked(&mut self, data: VdSynPhraseData, expected: &str) -> VdSynPhraseIdx {
        let phrase = self.phrase_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_phrase(phrase);
        assert_eq!(&formatter.finish(), expected);
        phrase
    }

    #[track_caller]
    pub fn new_clause_checked(&mut self, data: VdSynClauseData, expected: &str) -> VdSynClauseIdx {
        let clause = self.clause_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_clause(clause);
        assert_eq!(&formatter.finish(), expected);
        clause
    }

    #[track_caller]
    pub fn new_sentence_checked(
        &mut self,
        data: VdSynSentenceData,
        expected: &str,
    ) -> VdSynSentenceIdx {
        let sentence = self.sentence_arena.alloc_one(data);
        let mut formatter = self.latex_formatter();
        formatter.fmt_sentence(sentence);
        assert_eq!(&formatter.finish(), expected);
        sentence
    }

    pub fn finish(self) -> VdSynExprRegionData {
        VdSynExprRegionData::new(
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
        )
    }
}
