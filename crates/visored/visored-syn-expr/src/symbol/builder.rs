use super::*;
use clause::VdSynClauseArenaRef;
use defn::VdSynSymbolDefns;
use division::VdSynDivisionArenaRef;
use expr::{VdSynExprArenaRef, VdSynExprMap};
use phrase::VdSynPhraseArenaRef;
use resolution::{VdSynSymbolResolution, VdSynSymbolResolutions};
use sentence::VdSynSentenceArenaRef;
use stmt::VdSynStmtArenaRef;

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
        }
    }
}

impl<'a> VdSynSymbolBuilder<'a> {
    pub(super) fn build_all(&mut self) {
        todo!()
    }

    pub(super) fn finish(self) -> (VdSynSymbolDefns, VdSynSymbolResolutions) {
        (self.symbol_defns, self.symbol_resolutions)
    }
}
