use latex_ast::ast::{LxAstArena, LxAstArenaRef};
use latex_token::storage::LxTokenStorage;
use visored_annotation::annotations::VdAnnotations;

use crate::{
    clause::VdSynClauseArena,
    expr::{VdSynExprArena, VdSynExprData, VdSynExprIdx},
    phrase::{VdSynPhraseArena, VdSynPhraseData, VdSynPhraseIdx},
    region::VdSynExprRegionData,
    sentence::{VdSynSentenceArena, VdSynSentenceData, VdSynSentenceIdx},
};

pub(crate) struct VdSynExprBuilder<'db> {
    db: &'db ::salsa::Db,
    token_storage: &'db LxTokenStorage,
    ast_arena: LxAstArenaRef<'db>,
    annotations: &'db VdAnnotations,
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn new(
        db: &'db ::salsa::Db,
        token_storage: &'db LxTokenStorage,
        ast_arena: &'db LxAstArena,
        annotations: &'db VdAnnotations,
    ) -> Self {
        Self {
            db,
            token_storage,
            ast_arena: ast_arena.as_arena_ref(),
            annotations,
            expr_arena: Default::default(),
            phrase_arena: Default::default(),
            clause_arena: Default::default(),
            sentence_arena: Default::default(),
        }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn finish(self) -> VdSynExprRegionData {
        VdSynExprRegionData::new(
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
        )
    }
}
pub trait ToVdSyn<T> {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> T;
}
