pub(crate) mod builder;
pub mod lineage;
pub mod local_defn;
pub mod resolution;
pub mod scope;

use self::{builder::*, local_defn::*, resolution::*};
use crate::{
    block::VdSynBlockArenaRef, clause::VdSynClauseArenaRef, division::VdSynDivisionArenaRef,
    expr::VdSynExprArenaRef, phrase::VdSynPhraseArenaRef, range::*,
    sentence::VdSynSentenceArenaRef, *,
};
use block::{VdSynBlockIdxRange, VdSynBlockMap};
use clause::{r#let::VdSynLetClauseResolution, VdSynClauseMap};
use division::VdSynDivisionMap;
use entity_tree::VdSynExprEntityTreeNode;
use eterned::db::EternerDb;
use helpers::tracker::IsVdSynOutput;
use latex_math_letter::letter::LxMathLetter;
use visored_entity_path::module::VdModulePath;

pub enum VdSynSymbol {
    Letter(LxMathLetter),
}

pub struct VdSynExprVariableIdx {}

pub struct VdSynExprVariableData {}

pub(crate) fn build_all_symbol_defns_and_resolutions_with(
    db: &EternerDb,
    token_storage: &LxTokenStorage,
    ast_arena: LxAstArenaRef,
    ast_token_idx_range_map: &LxAstTokenIdxRangeMap,
    annotations: &VdAnnotations,
    default_resolution_table: &VdDefaultGlobalResolutionTable,
    expr_arena: VdSynExprArenaRef,
    phrase_arena: VdSynPhraseArenaRef,
    clause_arena: VdSynClauseArenaRef,
    sentence_arena: VdSynSentenceArenaRef,
    stmt_arena: VdSynBlockArenaRef,
    division_arena: VdSynDivisionArenaRef,
    expr_range_map: &VdSynExprTokenIdxRangeMap,
    phrase_range_map: &VdSynPhraseTokenIdxRangeMap,
    clause_range_map: &VdSynClauseTokenIdxRangeMap,
    sentence_range_map: &VdSynSentenceTokenIdxRangeMap,
    stmt_range_map: &VdSynBlockTokenIdxRangeMap,
    division_range_map: &VdSynDivisionTokenIdxRangeMap,
    root_node: &VdSynExprEntityTreeNode,
    stmt_entity_tree_node_map: &VdSynBlockMap<VdSynExprEntityTreeNode>,
    division_entity_tree_node_map: &VdSynDivisionMap<VdSynExprEntityTreeNode>,
    t: impl IsVdSynOutput,
) -> (
    VdSynClauseMap<VdSynLetClauseResolution>,
    VdSynSymbolLocalDefnStorage,
    VdSynSymbolResolutionsTable,
) {
    let mut symbol_builder = VdSynSymbolBuilder::new(
        db,
        default_resolution_table,
        expr_arena,
        phrase_arena,
        clause_arena,
        sentence_arena,
        stmt_arena,
        division_arena,
        expr_range_map,
        phrase_range_map,
        clause_range_map,
        sentence_range_map,
        stmt_range_map,
        division_range_map,
        root_node,
        stmt_entity_tree_node_map,
        division_entity_tree_node_map,
    );
    t.build_all_symbols(&mut symbol_builder);
    symbol_builder.finish()
}
