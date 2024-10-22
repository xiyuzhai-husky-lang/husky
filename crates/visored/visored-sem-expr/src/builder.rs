use crate::{
    clause::{VdSemClauseArena, VdSemClauseData, VdSemClauseIdx},
    expr::{VdSemExprArena, VdSemExprData, VdSemExprIdx},
    helpers::latex_fmt::VdSemExprLaTeXFormatter,
    phrase::{VdSemPhraseArena, VdSemPhraseData, VdSemPhraseIdx},
    region::VdSemExprRegionData,
    sentence::{VdSemSentenceArena, VdSemSentenceData, VdSemSentenceIdx},
};

pub(crate) struct VdSemExprBuilder<'db> {
    db: &'db ::salsa::Db,
    expr_arena: VdSemExprArena,
    phrase_arena: VdSemPhraseArena,
    clause_arena: VdSemClauseArena,
    sentence_arena: VdSemSentenceArena,
}

impl<'db> VdSemExprBuilder<'db> {
    pub fn finish(self) -> VdSemExprRegionData {
        VdSemExprRegionData::new(
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
        )
    }
}
