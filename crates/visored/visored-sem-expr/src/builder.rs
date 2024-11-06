use latex_token::storage::LxTokenStorage;
use visored_annotation::annotations::VdAnnotations;
use visored_resolution::table::VdDefaultResolutionTable;

use crate::{
    clause::{VdSemClauseArena, VdSemClauseData, VdSemClauseIdx},
    division::VdSemDivisionArena,
    expr::{VdSemExprArena, VdSemExprData, VdSemExprIdx},
    helpers::latex_fmt::VdSemExprLaTeXFormatter,
    phrase::{VdSemPhraseArena, VdSemPhraseData, VdSemPhraseIdx},
    region::VdSemExprRegionData,
    sentence::{VdSemSentenceArena, VdSemSentenceData, VdSemSentenceIdx},
    stmt::VdSemStmtArena,
};

pub(crate) struct VdSemExprBuilder<'db> {
    db: &'db ::salsa::Db,
    token_storage: &'db LxTokenStorage,
    annotations: &'db VdAnnotations,
    default_resolution_table: &'db VdDefaultResolutionTable,
    expr_arena: VdSemExprArena,
    phrase_arena: VdSemPhraseArena,
    clause_arena: VdSemClauseArena,
    sentence_arena: VdSemSentenceArena,
    stmt_arena: VdSemStmtArena,
    division_arena: VdSemDivisionArena,
}

impl<'db> VdSemExprBuilder<'db> {
    pub(crate) fn new(
        db: &'db ::salsa::Db,
        token_storage: &'db LxTokenStorage,
        annotations: &'db VdAnnotations,
        default_resolution_table: &'db VdDefaultResolutionTable,
    ) -> Self {
        Self {
            db,
            token_storage,
            annotations,
            default_resolution_table,
            expr_arena: VdSemExprArena::default(),
            phrase_arena: VdSemPhraseArena::default(),
            clause_arena: VdSemClauseArena::default(),
            sentence_arena: VdSemSentenceArena::default(),
            stmt_arena: VdSemStmtArena::default(),
            division_arena: VdSemDivisionArena::default(),
        }
    }

    pub fn finish_into_region_data(self) -> VdSemExprRegionData {
        VdSemExprRegionData::new(
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
        )
    }

    pub fn finish(
        self,
    ) -> (
        VdSemExprArena,
        VdSemPhraseArena,
        VdSemClauseArena,
        VdSemSentenceArena,
        VdSemStmtArena,
        VdSemDivisionArena,
    ) {
        (
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
            self.stmt_arena,
            self.division_arena,
        )
    }
}
