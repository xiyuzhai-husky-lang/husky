use latex_token::storage::LxTokenStorage;
use visored_annotation::annotations::VdAnnotations;
use visored_resolution::table::VdDefaultResolutionTable;
use visored_syn_expr::{
    clause::{VdSynClauseArenaRef, VdSynClauseIdx, VdSynClauseMap},
    division::{VdSynDivisionArenaRef, VdSynDivisionIdx, VdSynDivisionMap},
    expr::{VdSynExprArenaRef, VdSynExprIdx, VdSynExprMap},
    phrase::{VdSynPhraseArenaRef, VdSynPhraseIdx, VdSynPhraseMap},
    sentence::{VdSynSentenceArenaRef, VdSynSentenceIdx, VdSynSentenceMap},
    stmt::{VdSynStmtArenaRef, VdSynStmtIdx, VdSynStmtMap},
};

use crate::{
    clause::{VdSemClauseArena, VdSemClauseArenaRef, VdSemClauseData, VdSemClauseIdx},
    division::{VdSemDivisionArena, VdSemDivisionArenaRef, VdSemDivisionData, VdSemDivisionIdx},
    expr::{VdSemExprArena, VdSemExprArenaRef, VdSemExprData, VdSemExprIdx},
    helpers::latex_fmt::VdSemExprLaTeXFormatter,
    phrase::{VdSemPhraseArena, VdSemPhraseArenaRef, VdSemPhraseData, VdSemPhraseIdx},
    region::VdSemExprRegionData,
    sentence::{VdSemSentenceArena, VdSemSentenceArenaRef, VdSemSentenceData, VdSemSentenceIdx},
    stmt::{VdSemStmtArena, VdSemStmtArenaRef, VdSemStmtData, VdSemStmtIdx},
};

pub(crate) struct VdSemExprBuilder<'db> {
    db: &'db ::salsa::Db,
    token_storage: &'db LxTokenStorage,
    annotations: &'db VdAnnotations,
    default_resolution_table: &'db VdDefaultResolutionTable,
    syn_expr_arena: VdSynExprArenaRef<'db>,
    syn_phrase_arena: VdSynPhraseArenaRef<'db>,
    syn_clause_arena: VdSynClauseArenaRef<'db>,
    syn_sentence_arena: VdSynSentenceArenaRef<'db>,
    syn_stmt_arena: VdSynStmtArenaRef<'db>,
    syn_division_arena: VdSynDivisionArenaRef<'db>,
    expr_arena: VdSemExprArena,
    phrase_arena: VdSemPhraseArena,
    clause_arena: VdSemClauseArena,
    sentence_arena: VdSemSentenceArena,
    stmt_arena: VdSemStmtArena,
    division_arena: VdSemDivisionArena,
    syn_to_sem_expr_map: VdSynExprMap<VdSemExprIdx>,
    syn_to_sem_phrase_map: VdSynPhraseMap<VdSemPhraseIdx>,
    syn_to_sem_clause_map: VdSynClauseMap<VdSemClauseIdx>,
    syn_to_sem_sentence_map: VdSynSentenceMap<VdSemSentenceIdx>,
    syn_to_sem_stmt_map: VdSynStmtMap<VdSemStmtIdx>,
    syn_to_sem_division_map: VdSynDivisionMap<VdSemDivisionIdx>,
}

impl<'db> VdSemExprBuilder<'db> {
    pub(crate) fn new(
        db: &'db ::salsa::Db,
        token_storage: &'db LxTokenStorage,
        annotations: &'db VdAnnotations,
        default_resolution_table: &'db VdDefaultResolutionTable,
        syn_expr_arena: VdSynExprArenaRef<'db>,
        syn_phrase_arena: VdSynPhraseArenaRef<'db>,
        syn_clause_arena: VdSynClauseArenaRef<'db>,
        syn_sentence_arena: VdSynSentenceArenaRef<'db>,
        syn_stmt_arena: VdSynStmtArenaRef<'db>,
        syn_division_arena: VdSynDivisionArenaRef<'db>,
    ) -> Self {
        Self {
            db,
            token_storage,
            annotations,
            default_resolution_table,
            syn_expr_arena,
            syn_phrase_arena,
            syn_clause_arena,
            syn_sentence_arena,
            syn_stmt_arena,
            syn_division_arena,
            expr_arena: VdSemExprArena::default(),
            phrase_arena: VdSemPhraseArena::default(),
            clause_arena: VdSemClauseArena::default(),
            sentence_arena: VdSemSentenceArena::default(),
            stmt_arena: VdSemStmtArena::default(),
            division_arena: VdSemDivisionArena::default(),
            syn_to_sem_expr_map: VdSynExprMap::new2(syn_expr_arena),
            syn_to_sem_phrase_map: VdSynPhraseMap::new2(syn_phrase_arena),
            syn_to_sem_clause_map: VdSynClauseMap::new2(syn_clause_arena),
            syn_to_sem_sentence_map: VdSynSentenceMap::new2(syn_sentence_arena),
            syn_to_sem_stmt_map: VdSynStmtMap::new2(syn_stmt_arena),
            syn_to_sem_division_map: VdSynDivisionMap::new2(syn_division_arena),
        }
    }
}

/// # getters
impl<'db> VdSemExprBuilder<'db> {
    pub fn syn_expr_arena(&self) -> VdSynExprArenaRef<'db> {
        self.syn_expr_arena
    }

    pub fn syn_phrase_arena(&self) -> VdSynPhraseArenaRef<'db> {
        self.syn_phrase_arena
    }

    pub fn syn_clause_arena(&self) -> VdSynClauseArenaRef<'db> {
        self.syn_clause_arena
    }

    pub fn syn_sentence_arena(&self) -> VdSynSentenceArenaRef<'db> {
        self.syn_sentence_arena
    }

    pub fn syn_stmt_arena(&self) -> VdSynStmtArenaRef<'db> {
        self.syn_stmt_arena
    }

    pub fn syn_division_arena(&self) -> VdSynDivisionArenaRef<'db> {
        self.syn_division_arena
    }

    pub fn expr_arena(&self) -> VdSemExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn phrase_arena(&self) -> VdSemPhraseArenaRef {
        self.phrase_arena.as_arena_ref()
    }

    pub fn clause_arena(&self) -> VdSemClauseArenaRef {
        self.clause_arena.as_arena_ref()
    }

    pub fn sentence_arena(&self) -> VdSemSentenceArenaRef {
        self.sentence_arena.as_arena_ref()
    }

    pub fn stmt_arena(&self) -> VdSemStmtArenaRef {
        self.stmt_arena.as_arena_ref()
    }

    pub fn division_arena(&self) -> VdSemDivisionArenaRef {
        self.division_arena.as_arena_ref()
    }

    pub fn syn_to_sem_expr_map(&self) -> &VdSynExprMap<VdSemExprIdx> {
        &self.syn_to_sem_expr_map
    }

    pub fn syn_to_sem_phrase_map(&self) -> &VdSynPhraseMap<VdSemPhraseIdx> {
        &self.syn_to_sem_phrase_map
    }

    pub fn syn_to_sem_clause_map(&self) -> &VdSynClauseMap<VdSemClauseIdx> {
        &self.syn_to_sem_clause_map
    }

    pub fn syn_to_sem_sentence_map(&self) -> &VdSynSentenceMap<VdSemSentenceIdx> {
        &self.syn_to_sem_sentence_map
    }

    pub fn syn_to_sem_stmt_map(&self) -> &VdSynStmtMap<VdSemStmtIdx> {
        &self.syn_to_sem_stmt_map
    }

    pub fn syn_to_sem_division_map(&self) -> &VdSynDivisionMap<VdSemDivisionIdx> {
        &self.syn_to_sem_division_map
    }
}

impl<'db> VdSemExprBuilder<'db> {
    pub fn alloc_expr(&mut self, syn_expr: VdSynExprIdx, data: VdSemExprData) -> VdSemExprIdx {
        let expr = self.expr_arena.alloc_one(data);
        self.syn_to_sem_expr_map.insert(syn_expr, expr);
        expr
    }

    pub fn alloc_phrase(
        &mut self,
        syn_phrase: VdSynPhraseIdx,
        data: VdSemPhraseData,
    ) -> VdSemPhraseIdx {
        let phrase = self.phrase_arena.alloc_one(data);
        self.syn_to_sem_phrase_map.insert(syn_phrase, phrase);
        phrase
    }

    pub fn alloc_clause(
        &mut self,
        syn_clause: VdSynClauseIdx,
        data: VdSemClauseData,
    ) -> VdSemClauseIdx {
        let clause = self.clause_arena.alloc_one(data);
        self.syn_to_sem_clause_map.insert(syn_clause, clause);
        clause
    }

    pub fn alloc_sentence(
        &mut self,
        syn_sentence: VdSynSentenceIdx,
        data: VdSemSentenceData,
    ) -> VdSemSentenceIdx {
        let sentence = self.sentence_arena.alloc_one(data);
        self.syn_to_sem_sentence_map.insert(syn_sentence, sentence);
        sentence
    }

    pub fn alloc_stmt(&mut self, syn_stmt: VdSynStmtIdx, data: VdSemStmtData) -> VdSemStmtIdx {
        let stmt = self.stmt_arena.alloc_one(data);
        self.syn_to_sem_stmt_map.insert(syn_stmt, stmt);
        stmt
    }

    pub fn alloc_division(
        &mut self,
        syn_division: VdSynDivisionIdx,
        data: VdSemDivisionData,
    ) -> VdSemDivisionIdx {
        let division = self.division_arena.alloc_one(data);
        self.syn_to_sem_division_map.insert(syn_division, division);
        division
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
