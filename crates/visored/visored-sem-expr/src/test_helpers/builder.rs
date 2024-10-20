use crate::{
    clause::VdSemClauseArena, expr::VdSemExprArena, phrase::VdSemPhraseArena,
    sentence::VdSemSentenceArena,
};

pub struct VdSemExprTestBuilder<'db> {
    db: &'db ::salsa::Db,
    arena: VdSemExprArena,
    phrase_arena: VdSemPhraseArena,
    clause_arena: VdSemClauseArena,
    sentence_arena: VdSemSentenceArena,
}
