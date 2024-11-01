use crate::{
    clause::VdSynClauseArena,
    expr::{VdSynExprArena, VdSynExprData, VdSynExprIdx, VdSynExprIdxRange},
    phrase::{VdSynPhraseArena, VdSynPhraseData, VdSynPhraseIdx},
    range::{
        VdSynClauseTokenIdxRangeMap, VdSynExprTokenIdxRange, VdSynExprTokenIdxRangeMap,
        VdSynPhraseTokenIdxRangeMap, VdSynSentenceTokenIdxRangeMap,
    },
    region::VdSynExprRegionData,
    sentence::{VdSynSentenceArena, VdSynSentenceData, VdSynSentenceIdx},
};
use either::*;
use latex_ast::{
    ast::{LxAstArena, LxAstArenaRef, LxAstIdxRange},
    range::LxAstTokenIdxRangeMap,
};
use latex_token::storage::LxTokenStorage;
use visored_annotation::annotations::VdAnnotations;

pub(crate) struct VdSynExprBuilder<'db> {
    db: &'db ::salsa::Db,
    token_storage: &'db LxTokenStorage,
    ast_arena: LxAstArenaRef<'db>,
    ast_token_idx_range_map: &'db LxAstTokenIdxRangeMap,
    annotations: &'db VdAnnotations,
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
}

/// # constructor
impl<'db> VdSynExprBuilder<'db> {
    pub fn new(
        db: &'db ::salsa::Db,
        token_storage: &'db LxTokenStorage,
        ast_arena: &'db LxAstArena,
        ast_token_idx_range_map: &'db LxAstTokenIdxRangeMap,
        annotations: &'db VdAnnotations,
    ) -> Self {
        Self {
            db,
            token_storage,
            ast_arena: ast_arena.as_arena_ref(),
            ast_token_idx_range_map,
            annotations,
            expr_arena: Default::default(),
            phrase_arena: Default::default(),
            clause_arena: Default::default(),
            sentence_arena: Default::default(),
        }
    }

    pub(crate) fn alloc_expr(&mut self, data: VdSynExprData) -> VdSynExprIdx {
        self.expr_arena.alloc_one(data)
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

    pub(crate) fn ast_token_idx_range_map(&self) -> &LxAstTokenIdxRangeMap {
        &self.ast_token_idx_range_map
    }

    pub(crate) fn annotations(&self) -> &VdAnnotations {
        self.annotations
    }

    pub(crate) fn expr_arena(&self) -> &VdSynExprArena {
        &self.expr_arena
    }

    pub(crate) fn phrase_arena(&self) -> &VdSynPhraseArena {
        &self.phrase_arena
    }

    pub(crate) fn clause_arena(&self) -> &VdSynClauseArena {
        &self.clause_arena
    }

    pub(crate) fn sentence_arena(&self) -> &VdSynSentenceArena {
        &self.sentence_arena
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
        VdSynPhraseArena,
        VdSynClauseArena,
        VdSynSentenceArena,
    ) {
        (
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

impl ToVdSyn<Either<VdSynExprIdx, ()>> for LxAstIdxRange {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> Either<VdSynExprIdx, ()> {
        match self {
            LxAstIdxRange::Math(slf) => Either::Left(slf.to_vd_syn(builder)),
            LxAstIdxRange::Rose(slf) => Either::Right(todo!()),
        }
    }
}
