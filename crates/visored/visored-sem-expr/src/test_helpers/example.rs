use super::*;
use crate::range::{calc_expr_range_map, VdSemStmtTokenIdxRangeMap};
use crate::stmt::{VdSemStmtArena, VdSemStmtIdxRange};
use crate::{
    builder::VdSemExprBuilder,
    clause::VdSemClauseArena,
    expr::VdSemExprArena,
    phrase::VdSemPhraseArena,
    range::{
        VdSemClauseTokenIdxRangeMap, VdSemExprTokenIdxRangeMap, VdSemPhraseTokenIdxRangeMap,
        VdSemSentenceTokenIdxRangeMap,
    },
    sentence::VdSemSentenceArena,
};
use crate::{division::VdSemDivisionArena, expr::VdSemExprIdx};
use crate::{
    helpers::show::display_tree::VdSemExprDisplayTreeBuilder, range::VdSemDivisionTokenIdxRangeMap,
};
use either::*;
use husky_tree_utils::display::DisplayTree;
use latex_ast::{
    ast::{parse_latex_input_into_asts, rose::LxRoseAstIdxRange, LxAstArena, LxAstIdxRange},
    range::{calc_ast_token_idx_range_map, LxAstTokenIdxRangeMap},
};
use latex_command::signature::table::LxCommandSignatureTable;
use latex_prelude::mode::LxMode;
use latex_token::{idx::LxTokenIdxRange, storage::LxTokenStorage};
use symbol::local_defn::storage::VdSemSymbolLocalDefnStorage;
use visored_annotation::{
    annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation},
    annotations::VdAnnotations,
};
use visored_global_dispatch::default_table::VdDefaultGlobalDispatchTable;
use visored_global_resolution::default_table::VdDefaultGlobalResolutionTable;
use visored_syn_expr::{
    clause::VdSynClauseArena,
    division::VdSynDivisionArena,
    expr::VdSynExprArena,
    phrase::VdSynPhraseArena,
    range::{
        VdSynClauseTokenIdxRangeMap, VdSynDivisionTokenIdxRangeMap, VdSynExprTokenIdxRangeMap,
        VdSynPhraseTokenIdxRangeMap, VdSynSentenceTokenIdxRangeMap, VdSynStmtTokenIdxRangeMap,
    },
    sentence::VdSynSentenceArena,
    stmt::VdSynStmtArena,
    test_helpers::example::VdSynExprExample,
};
use visored_term::ty::table::VdItemPathZfcTypeTable;

pub struct VdSemExprExample {
    pub input: String,
    pub root_mode: LxMode,
    pub annotations: VdAnnotations,
    pub default_resolution_table: VdDefaultGlobalResolutionTable,
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub asts: LxAstIdxRange,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
    pub expr_arena: VdSemExprArena,
    pub phrase_arena: VdSemPhraseArena,
    pub clause_arena: VdSemClauseArena,
    pub sentence_arena: VdSemSentenceArena,
    pub stmt_arena: VdSemStmtArena,
    pub division_arena: VdSemDivisionArena,
    pub expr_range_map: VdSemExprTokenIdxRangeMap,
    pub phrase_range_map: VdSemPhraseTokenIdxRangeMap,
    pub clause_range_map: VdSemClauseTokenIdxRangeMap,
    pub sentence_range_map: VdSemSentenceTokenIdxRangeMap,
    pub stmt_range_map: VdSemStmtTokenIdxRangeMap,
    pub division_range_map: VdSemDivisionTokenIdxRangeMap,
    pub symbol_local_defn_storage: VdSemSymbolLocalDefnStorage,
    pub result: Either<VdSemExprIdx, VdSemStmtIdxRange>,
}

impl VdSemExprExample {
    pub fn new(
        input: &str,
        root_mode: LxMode,
        token_annotations: &[((&str, &str), VdTokenAnnotation)],
        space_annotations: &[((&str, &str), VdSpaceAnnotation)],
        db: &salsa::Db,
    ) -> Self {
        let VdSynExprExample {
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            annotations,
            default_resolution_table,
            input,
            root_mode,
            asts,
            result: syn_result,
            expr_arena: syn_expr_arena,
            phrase_arena: syn_phrase_arena,
            clause_arena: syn_clause_arena,
            sentence_arena: syn_sentence_arena,
            stmt_arena: syn_stmt_arena,
            division_arena: syn_division_arena,
            expr_range_map: syn_expr_range_map,
            phrase_range_map: syn_phrase_range_map,
            clause_range_map: syn_clause_range_map,
            sentence_range_map: syn_sentence_range_map,
            stmt_range_map: syn_stmt_range_map,
            division_range_map: syn_division_range_map,
            symbol_local_defn_storage: syn_symbol_local_defn_storage,
            symbol_resolution_table: syn_symbol_resolution_table,
        } = VdSynExprExample::new(input, root_mode, token_annotations, space_annotations, db);
        let item_path_zfc_ty_table = VdItemPathZfcTypeTable::new_standard(db);
        let default_global_dispatch_table = VdDefaultGlobalDispatchTable::new_standard(db);
        let mut builder = VdSemExprBuilder::new(
            db,
            &token_storage,
            &annotations,
            &default_resolution_table,
            syn_expr_arena.as_arena_ref(),
            syn_phrase_arena.as_arena_ref(),
            syn_clause_arena.as_arena_ref(),
            syn_sentence_arena.as_arena_ref(),
            syn_stmt_arena.as_arena_ref(),
            syn_division_arena.as_arena_ref(),
            &syn_symbol_local_defn_storage,
            &syn_symbol_resolution_table,
            &item_path_zfc_ty_table,
            &default_global_dispatch_table,
        );
        let result = syn_result.to_vd_sem(&mut builder);
        let (
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            symbol_local_defn_storage,
        ) = builder.finish();
        let (
            expr_range_map,
            phrase_range_map,
            clause_range_map,
            sentence_range_map,
            stmt_range_map,
            division_range_map,
        ) = calc_expr_range_map(
            db,
            &expr_arena,
            &phrase_arena,
            &clause_arena,
            &sentence_arena,
            &stmt_arena,
            &division_arena,
        );
        Self {
            input: input.to_string(),
            root_mode,
            annotations,
            default_resolution_table,
            token_storage,
            ast_arena,
            asts,
            ast_token_idx_range_map,
            result,
            expr_arena,
            phrase_arena,
            clause_arena,
            sentence_arena,
            stmt_arena,
            division_arena,
            symbol_local_defn_storage,
            expr_range_map,
            phrase_range_map,
            clause_range_map,
            sentence_range_map,
            stmt_range_map,
            division_range_map,
        }
    }

    pub(crate) fn show_display_tree(&self, db: &salsa::Db) -> String {
        let builder = VdSemExprDisplayTreeBuilder::new(
            db,
            &self.input,
            &self.token_storage,
            self.ast_arena.as_arena_ref(),
            &self.ast_token_idx_range_map,
            self.expr_arena.as_arena_ref(),
            self.phrase_arena.as_arena_ref(),
            self.clause_arena.as_arena_ref(),
            self.sentence_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.division_arena.as_arena_ref(),
            &self.expr_range_map,
            &self.phrase_range_map,
            &self.clause_range_map,
            &self.sentence_range_map,
            &self.stmt_range_map,
            &self.division_range_map,
        );
        match self.result {
            Left(expr) => builder.render_expr(expr).show(&Default::default()),
            Right(stmts) => builder.render_all_stmts(stmts).show(&Default::default()),
        }
    }
}
