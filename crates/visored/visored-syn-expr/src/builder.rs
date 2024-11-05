use crate::{
    clause::{VdSynClauseArena, VdSynClauseData, VdSynClauseIdx, VdSynClauseIdxRange},
    expr::{VdSynExprArena, VdSynExprData, VdSynExprIdx, VdSynExprIdxRange},
    phrase::{VdSynPhraseArena, VdSynPhraseData, VdSynPhraseIdx, VdSynPhraseIdxRange},
    range::{
        VdSynClauseTokenIdxRangeMap, VdSynExprTokenIdxRange, VdSynExprTokenIdxRangeMap,
        VdSynPhraseTokenIdxRangeMap, VdSynSentenceTokenIdxRangeMap,
    },
    region::VdSynExprRegionData,
    sentence::{VdSynSentenceArena, VdSynSentenceData, VdSynSentenceIdx, VdSynSentenceIdxRange},
    stmt::{VdSynStmtArena, VdSynStmtData, VdSynStmtIdx, VdSynStmtIdxRange},
};
use either::*;
use latex_ast::{
    ast::{rose::LxRoseAstIdxRange, LxAstArena, LxAstArenaRef, LxAstIdxRange},
    range::LxAstTokenIdxRangeMap,
};
use latex_token::{idx::LxTokenIdxRange, storage::LxTokenStorage};
use visored_annotation::annotations::VdAnnotations;
use visored_resolution::table::VdDefaultResolutionTable;

pub(crate) struct VdSynExprBuilder<'db> {
    db: &'db ::salsa::Db,
    token_storage: &'db LxTokenStorage,
    ast_arena: LxAstArenaRef<'db>,
    ast_token_idx_range_map: &'db LxAstTokenIdxRangeMap,
    annotations: &'db VdAnnotations,
    default_resolution_table: &'db VdDefaultResolutionTable,
    expr_arena: VdSynExprArena,
    phrase_arena: VdSynPhraseArena,
    clause_arena: VdSynClauseArena,
    sentence_arena: VdSynSentenceArena,
    stmt_arena: VdSynStmtArena,
}

/// # constructor
impl<'db> VdSynExprBuilder<'db> {
    pub fn new(
        db: &'db ::salsa::Db,
        token_storage: &'db LxTokenStorage,
        ast_arena: &'db LxAstArena,
        ast_token_idx_range_map: &'db LxAstTokenIdxRangeMap,
        annotations: &'db VdAnnotations,
        default_resolution_table: &'db VdDefaultResolutionTable,
    ) -> Self {
        Self {
            db,
            token_storage,
            ast_arena: ast_arena.as_arena_ref(),
            ast_token_idx_range_map,
            annotations,
            default_resolution_table,
            expr_arena: Default::default(),
            phrase_arena: Default::default(),
            clause_arena: Default::default(),
            sentence_arena: Default::default(),
            stmt_arena: Default::default(),
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

    pub(crate) fn ast_arena(&self) -> LxAstArenaRef<'db> {
        self.ast_arena
    }

    pub(crate) fn ast_token_idx_range_map(&self) -> &LxAstTokenIdxRangeMap {
        &self.ast_token_idx_range_map
    }

    pub(crate) fn annotations(&self) -> &VdAnnotations {
        self.annotations
    }

    pub(crate) fn default_resolution_table(&self) -> &VdDefaultResolutionTable {
        self.default_resolution_table
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

/// # actions
impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn alloc_expr(&mut self, data: VdSynExprData) -> VdSynExprIdx {
        self.expr_arena.alloc_one(data)
    }

    pub(crate) fn alloc_exprs(&mut self, data: Vec<VdSynExprData>) -> VdSynExprIdxRange {
        self.expr_arena.alloc_batch(data)
    }

    pub(crate) fn alloc_phrase(&mut self, data: VdSynPhraseData) -> VdSynPhraseIdx {
        self.phrase_arena.alloc_one(data)
    }

    pub(crate) fn alloc_phrases(&mut self, data: Vec<VdSynPhraseData>) -> VdSynPhraseIdxRange {
        self.phrase_arena.alloc_batch(data)
    }

    pub(crate) fn alloc_clause(&mut self, data: VdSynClauseData) -> VdSynClauseIdx {
        self.clause_arena.alloc_one(data)
    }

    pub(crate) fn alloc_clauses(&mut self, data: Vec<VdSynClauseData>) -> VdSynClauseIdxRange {
        self.clause_arena.alloc_batch(data)
    }

    pub(crate) fn alloc_sentence(&mut self, data: VdSynSentenceData) -> VdSynSentenceIdx {
        self.sentence_arena.alloc_one(data)
    }

    pub(crate) fn alloc_sentences(
        &mut self,
        data: Vec<VdSynSentenceData>,
    ) -> VdSynSentenceIdxRange {
        self.sentence_arena.alloc_batch(data)
    }

    pub(crate) fn alloc_stmt(&mut self, data: VdSynStmtData) -> VdSynStmtIdx {
        self.stmt_arena.alloc_one(data)
    }

    pub(crate) fn alloc_stmts(&mut self, data: Vec<VdSynStmtData>) -> VdSynStmtIdxRange {
        self.stmt_arena.alloc_batch(data)
    }

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
        VdSynStmtArena,
    ) {
        (
            self.expr_arena,
            self.phrase_arena,
            self.clause_arena,
            self.sentence_arena,
            self.stmt_arena,
        )
    }
}
pub trait ToVdSyn<T> {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> T;
}

impl<R> ToVdSyn<Either<VdSynExprIdx, R>> for (LxTokenIdxRange, LxAstIdxRange)
where
    (LxTokenIdxRange, LxRoseAstIdxRange): ToVdSyn<R>,
{
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> Either<VdSynExprIdx, R> {
        let (token_range, asts) = self;
        match asts {
            LxAstIdxRange::Math(asts) => Either::Left((token_range, asts).to_vd_syn(builder)),
            LxAstIdxRange::Rose(asts) => Either::Right((token_range, asts).to_vd_syn(builder)),
        }
    }
}
