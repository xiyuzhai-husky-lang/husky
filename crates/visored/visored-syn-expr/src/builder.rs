use crate::{
    clause::VdSynClauseArena,
    expr::{VdSynExprArena, VdSynExprData, VdSynExprIdx},
    phrase::{VdSynPhraseArena, VdSynPhraseData, VdSynPhraseIdx},
    range::{VdSynClauseRangeMap, VdSynExprRangeMap, VdSynPhraseRangeMap, VdSynSentenceRangeMap},
    region::VdSynExprRegionData,
    sentence::{VdSynSentenceArena, VdSynSentenceData, VdSynSentenceIdx},
};
use either::*;
use latex_ast::ast::{LxAstArena, LxAstArenaRef, LxAstIdxRange};
use latex_token::storage::LxTokenStorage;
use visored_annotation::annotations::VdAnnotations;

pub(crate) struct VdSynExprBuilder<'db> {
    db: &'db ::salsa::Db,
    token_storage: &'db LxTokenStorage,
    ast_arena: LxAstArenaRef<'db>,
    annotations: &'db VdAnnotations,
    expr_arena: VdSynExprArena,
    expr_range_map: VdSynExprRangeMap,
    phrase_arena: VdSynPhraseArena,
    phrase_range_map: VdSynPhraseRangeMap,
    clause_arena: VdSynClauseArena,
    clause_range_map: VdSynClauseRangeMap,
    sentence_arena: VdSynSentenceArena,
    sentence_range_map: VdSynSentenceRangeMap,
}

/// # constructor
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
            expr_range_map: Default::default(),
            phrase_range_map: Default::default(),
            clause_range_map: Default::default(),
            sentence_range_map: Default::default(),
        }
    }
}

/// # getters
impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn db(&self) -> &'db ::salsa::Db {
        self.db
    }

    pub(crate) fn token_storage(&self) -> &LxTokenStorage {
        self.token_storage
    }

    pub(crate) fn ast_arena(&self) -> &LxAstArenaRef<'db> {
        &self.ast_arena
    }

    pub(crate) fn annotations(&self) -> &VdAnnotations {
        self.annotations
    }

    pub(crate) fn expr_arena(&self) -> &VdSynExprArena {
        &self.expr_arena
    }

    pub(crate) fn expr_range_map(&self) -> &VdSynExprRangeMap {
        &self.expr_range_map
    }

    pub(crate) fn phrase_arena(&self) -> &VdSynPhraseArena {
        &self.phrase_arena
    }

    pub(crate) fn phrase_range_map(&self) -> &VdSynPhraseRangeMap {
        &self.phrase_range_map
    }

    pub(crate) fn clause_arena(&self) -> &VdSynClauseArena {
        &self.clause_arena
    }

    pub(crate) fn clause_range_map(&self) -> &VdSynClauseRangeMap {
        &self.clause_range_map
    }

    pub(crate) fn sentence_arena(&self) -> &VdSynSentenceArena {
        &self.sentence_arena
    }

    pub(crate) fn sentence_range_map(&self) -> &VdSynSentenceRangeMap {
        &self.sentence_range_map
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn finish_to_region_data(self) -> VdSynExprRegionData {
        VdSynExprRegionData::new(
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
        )
    }

    pub fn finish(
        self,
    ) -> (
        VdSynExprArena,
        VdSynExprRangeMap,
        VdSynPhraseArena,
        VdSynPhraseRangeMap,
        VdSynClauseArena,
        VdSynClauseRangeMap,
        VdSynSentenceArena,
        VdSynSentenceRangeMap,
    ) {
        (
            self.expr_arena,
            self.expr_range_map,
            self.phrase_arena,
            self.phrase_range_map,
            self.clause_arena,
            self.clause_range_map,
            self.sentence_arena,
            self.sentence_range_map,
        )
    }
}
pub trait ToVdSyn<T> {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> T;
}

impl ToVdSyn<Either<VdSynExprIdx, ()>> for LxAstIdxRange {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> Either<VdSynExprIdx, ()> {
        match self {
            LxAstIdxRange::Math(slf) => Either::Left(slf.to_vd_syn(builder)),
            LxAstIdxRange::Rose(slf) => Either::Right(todo!()),
        }
    }
}
