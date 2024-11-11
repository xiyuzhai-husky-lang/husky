use super::{
    defn::VdSynSymbolDefns,
    resolution::{VdSynSymbolResolution, VdSynSymbolResolutions},
    *,
};
use crate::{clause::*, division::*, expr::*, phrase::*, range::*, sentence::*, stmt::*};
use smallvec::{smallvec, SmallVec};

pub struct VdSynSymbolBuilder<'a> {
    db: &'a ::salsa::Db,
    token_storage: &'a LxTokenStorage,
    ast_arena: LxAstArenaRef<'a>,
    ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    annotations: &'a VdAnnotations,
    default_resolution_table: &'a VdDefaultResolutionTable,
    expr_arena: VdSynExprArenaRef<'a>,
    phrase_arena: VdSynPhraseArenaRef<'a>,
    clause_arena: VdSynClauseArenaRef<'a>,
    sentence_arena: VdSynSentenceArenaRef<'a>,
    stmt_arena: VdSynStmtArenaRef<'a>,
    division_arena: VdSynDivisionArenaRef<'a>,
    symbol_defns: VdSynSymbolDefns,
    symbol_resolutions: VdSynSymbolResolutions,
    // the order is from outer to inner
    current_divisions: SmallVec<[VdSynDivisionIdx; 8]>,
    current_stmts: SmallVec<[VdSynStmtIdx; 8]>,
    current_sentences: SmallVec<[VdSynSentenceIdx; 8]>,
    current_clauses: SmallVec<[VdSynClauseIdx; 8]>,
    current_phrases: SmallVec<[VdSynPhraseIdx; 8]>,
}

impl<'a> VdSynSymbolBuilder<'a> {
    pub fn new(
        db: &'a ::salsa::Db,
        token_storage: &'a LxTokenStorage,
        ast_arena: LxAstArenaRef<'a>,
        ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
        annotations: &'a VdAnnotations,
        default_resolution_table: &'a VdDefaultResolutionTable,
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
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            annotations,
            default_resolution_table,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            symbol_defns: VdSynSymbolDefns::default(),
            symbol_resolutions: VdSynSymbolResolutions::new(expr_arena),
            current_divisions: smallvec![],
            current_stmts: smallvec![],
            current_sentences: smallvec![],
            current_clauses: smallvec![],
            current_phrases: smallvec![],
        }
    }
}

impl<'a> VdSynSymbolBuilder<'a> {
    pub(super) fn build_stmts(&mut self, stmts: VdSynStmtIdxRange) {
        for stmt in stmts {
            self.build_stmt(stmt);
        }
    }

    fn build_stmt(&mut self, stmt: VdSynStmtIdx) {
        self.current_stmts.push(stmt);
        self.build_stmt_aux(stmt);
        self.current_stmts.pop();
    }

    fn build_stmt_aux(&mut self, stmt: VdSynStmtIdx) {
        match self.stmt_arena[stmt] {
            VdSynStmtData::Paragraph(sentences) => self.build_sentences(sentences),
            VdSynStmtData::Block { environment, stmts } => self.build_stmts(stmts),
        }
    }

    fn build_sentences(&mut self, sentences: VdSynSentenceIdxRange) {
        for sentence in sentences {
            self.build_sentence(sentence);
        }
    }

    fn build_sentence(&mut self, sentence: VdSynSentenceIdx) {
        self.current_sentences.push(sentence);
        self.build_sentence_aux(sentence);
        self.current_sentences.pop();
    }

    fn build_sentence_aux(&mut self, sentence: VdSynSentenceIdx) {
        match self.sentence_arena[sentence] {
            VdSynSentenceData::Clauses { clauses, .. } => self.build_clauses(clauses),
        }
    }

    fn build_clauses(&mut self, clauses: VdSynClauseIdxRange) {
        for clause in clauses {
            self.build_clause(clause);
        }
    }

    fn build_clause(&mut self, clause: VdSynClauseIdx) {
        self.current_clauses.push(clause);
        self.build_clause_aux(clause);
        self.current_clauses.pop();
    }

    fn build_clause_aux(&mut self, clause: VdSynClauseIdx) {
        match self.clause_arena[clause] {
            VdSynClauseData::Let { .. } => todo!(),
            VdSynClauseData::Assume {
                assume_token_idx,
                left_dollar_token_idx,
                formula,
                right_dollar_token_idx,
            } => todo!(),
            VdSynClauseData::Then {
                then_token_idx,
                left_dollar_token_idx,
                formula,
                right_dollar_token_idx,
            } => todo!(),
        }
    }

    pub(super) fn finish(self) -> (VdSynSymbolDefns, VdSynSymbolResolutions) {
        (self.symbol_defns, self.symbol_resolutions)
    }
}
