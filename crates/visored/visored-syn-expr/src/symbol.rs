mod builder;
pub mod defn;
pub mod resolution;
pub mod scope;

use self::{builder::*, defn::*, resolution::*};
use crate::{
    clause::VdSynClauseArenaRef, division::VdSynDivisionArenaRef, expr::VdSynExprArenaRef,
    phrase::VdSynPhraseArenaRef, sentence::VdSynSentenceArenaRef, stmt::VdSynStmtArenaRef, *,
};
use latex_math_letter::letter::LxMathLetter;

pub enum VdSynSymbol {
    Letter(LxMathLetter),
}

pub struct VdSynExprVariableIdx {}

pub struct VdSynExprVariableData {}

pub(crate) fn build_symbol_defns_and_resolutions(
    db: &::salsa::Db,
    token_storage: &LxTokenStorage,
    ast_arena: LxAstArenaRef,
    ast_token_idx_range_map: &LxAstTokenIdxRangeMap,
    annotations: &VdAnnotations,
    default_resolution_table: &VdDefaultResolutionTable,
    expr_arena: VdSynExprArenaRef,
    phrase_arena: VdSynPhraseArenaRef,
    clause_arena: VdSynClauseArenaRef,
    sentence_arena: VdSynSentenceArenaRef,
    stmt_arena: VdSynStmtArenaRef,
    division_arena: VdSynDivisionArenaRef,
) -> (VdSynSymbolDefns, VdSynSymbolResolutions) {
    let mut symbol_builder = VdSynSymbolBuilder::new(
        db,
        &token_storage,
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
    );
    symbol_builder.build_all();
    symbol_builder.finish()
}
