use super::{
    lineage::VdSynLineage,
    local_defn::VdSynSymbolLocalDefnStorage,
    resolution::{VdSynSymbolResolution, VdSynSymbolResolutionsTable},
    *,
};
use crate::{clause::*, division::*, expr::*, phrase::*, range::*, sentence::*, stmt::*};
use r#let::{
    assigned::VdSynLetAssignedResolution, placeholder::VdSynLetPlaceholderResolution,
    VdSynLetClauseResolution,
};
use smallvec::{smallvec, SmallVec};

pub struct VdSynSymbolBuilder<'a> {
    db: &'a ::salsa::Db,
    default_global_resolution_table: &'a VdDefaultGlobalResolutionTable,
    expr_arena: VdSynExprArenaRef<'a>,
    phrase_arena: VdSynPhraseArenaRef<'a>,
    clause_arena: VdSynClauseArenaRef<'a>,
    sentence_arena: VdSynSentenceArenaRef<'a>,
    stmt_arena: VdSynStmtArenaRef<'a>,
    division_arena: VdSynDivisionArenaRef<'a>,
    symbol_local_defn_table: VdSynSymbolLocalDefnStorage,
    symbol_resolutions_table: VdSynSymbolResolutionsTable,
    lineage: VdSynLineage,
}

impl<'a> VdSynSymbolBuilder<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        default_global_resolution_table: &'a VdDefaultGlobalResolutionTable,
        expr_arena: VdSynExprArenaRef<'a>,
        phrase_arena: VdSynPhraseArenaRef<'a>,
        clause_arena: VdSynClauseArenaRef<'a>,
        sentence_arena: VdSynSentenceArenaRef<'a>,
        stmt_arena: VdSynStmtArenaRef<'a>,
        division_arena: VdSynDivisionArenaRef<'a>,
        expr_range_map: &'a VdSynExprTokenIdxRangeMap,
        phrase_range_map: &'a VdSynPhraseTokenIdxRangeMap,
        clause_range_map: &'a VdSynClauseTokenIdxRangeMap,
        sentence_range_map: &'a VdSynSentenceTokenIdxRangeMap,
        stmt_range_map: &'a VdSynStmtTokenIdxRangeMap,
        division_range_map: &'a VdSynDivisionTokenIdxRangeMap,
    ) -> Self {
        Self {
            db,
            default_global_resolution_table,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            symbol_local_defn_table: VdSynSymbolLocalDefnStorage::default(),
            symbol_resolutions_table: VdSynSymbolResolutionsTable::new(expr_arena),
            lineage: VdSynLineage {
                divisions: smallvec![],
                stmts: smallvec![],
                sentence: None,
                clause: None,
                phrases: smallvec![],
                exprs: smallvec![],
            },
        }
    }
}

/// # getters
impl<'a> VdSynSymbolBuilder<'a> {
    pub(crate) fn default_global_resolution_table(&self) -> &VdDefaultGlobalResolutionTable {
        self.default_global_resolution_table
    }

    pub(crate) fn symbol_local_defn_table(&self) -> &VdSynSymbolLocalDefnStorage {
        &self.symbol_local_defn_table
    }

    pub(crate) fn expr_arena(&self) -> VdSynExprArenaRef<'a> {
        self.expr_arena
    }

    pub(crate) fn phrase_arena(&self) -> VdSynPhraseArenaRef<'a> {
        self.phrase_arena
    }

    pub(crate) fn clause_arena(&self) -> VdSynClauseArenaRef<'a> {
        self.clause_arena
    }

    pub(crate) fn sentence_arena(&self) -> VdSynSentenceArenaRef<'a> {
        self.sentence_arena
    }

    pub(crate) fn stmt_arena(&self) -> VdSynStmtArenaRef<'a> {
        self.stmt_arena
    }

    pub(crate) fn division_arena(&self) -> VdSynDivisionArenaRef<'a> {
        self.division_arena
    }
}

/// # actions
impl<'a> VdSynSymbolBuilder<'a> {
    pub(crate) fn build_stmts(&mut self, stmts: VdSynStmtIdxRange) {
        for stmt in stmts {
            self.build_stmt(stmt);
        }
    }

    pub(crate) fn build_stmt(&mut self, stmt: VdSynStmtIdx) {
        self.lineage.stmts.push(stmt);
        self.build_stmt_aux(stmt);
        self.lineage.stmts.pop();
    }

    pub(crate) fn build_sentences(&mut self, sentences: VdSynSentenceIdxRange) {
        for sentence in sentences {
            self.build_sentence(sentence);
        }
    }

    pub(crate) fn build_sentence(&mut self, sentence: VdSynSentenceIdx) {
        debug_assert!(self.lineage.sentence.is_none());
        self.lineage.sentence = Some(sentence);
        self.build_sentence_aux(sentence);
        self.lineage.sentence = None;
    }

    pub(crate) fn build_sentence_aux(&mut self, sentence: VdSynSentenceIdx) {
        match self.sentence_arena[sentence] {
            VdSynSentenceData::Clauses { clauses, .. } => self.build_clauses(clauses),
        }
    }

    pub(crate) fn build_clauses(&mut self, clauses: VdSynClauseIdxRange) {
        for clause in clauses {
            self.build_clause(clause);
        }
    }

    pub(crate) fn build_clause(&mut self, clause: VdSynClauseIdx) {
        debug_assert!(self.lineage.clause.is_none());
        self.lineage.clause = Some(clause);
        self.build_clause_aux(clause);
        self.lineage.clause = None;
    }

    pub(crate) fn build_phrase(&mut self, phrase: VdSynPhraseIdx) {
        self.lineage.phrases.push(phrase);
        self.build_phrase_aux(phrase);
        self.lineage.phrases.pop();
    }

    pub(crate) fn build_expr(&mut self, expr: VdSynExprIdx) {
        self.lineage.exprs.push(expr);
        let resolutions_result = self.build_expr_aux(expr);
        self.symbol_resolutions_table
            .add_expr_symbol_resolutions(expr, resolutions_result);
        self.lineage.exprs.pop();
    }

    pub(crate) fn define_symbol(
        &mut self,
        head: VdSynSymbolLocalDefnHead,
        body: VdSynSymbolLocalDefnBody,
        src: VdSynSymbolLocalDefnSrc,
    ) {
        self.symbol_local_defn_table
            .define_symbol(head, body, src, self.lineage.clone());
    }

    pub(super) fn finish(self) -> (VdSynSymbolLocalDefnStorage, VdSynSymbolResolutionsTable) {
        (self.symbol_local_defn_table, self.symbol_resolutions_table)
    }
}
