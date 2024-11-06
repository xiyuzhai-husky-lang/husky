use super::*;
use crate::{
    clause::VdSemClauseArenaRef,
    division::VdSemDivisionArenaRef,
    expr::{VdSemExprArenaRef, VdSemExprIdx},
    phrase::VdSemPhraseArenaRef,
    range::{
        VdSemClauseTokenIdxRangeMap, VdSemDivisionTokenIdxRangeMap, VdSemExprTokenIdxRangeMap,
        VdSemPhraseTokenIdxRangeMap, VdSemSentenceTokenIdxRangeMap, VdSemStmtTokenIdxRangeMap,
    },
    sentence::VdSemSentenceArenaRef,
    stmt::{VdSemStmtArenaRef, VdSemStmtIdxRange},
};
use husky_tree_utils::display::DisplayTree;
use latex_ast::{ast::LxAstArenaRef, range::LxAstTokenIdxRangeMap};
use latex_token::storage::LxTokenStorage;

pub struct VdSemExprDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    input: &'a str,
    token_storage: &'a LxTokenStorage,
    ast_arena: LxAstArenaRef<'a>,
    ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    expr_arena: VdSemExprArenaRef<'a>,
    phrase_arena: VdSemPhraseArenaRef<'a>,
    clause_arena: VdSemClauseArenaRef<'a>,
    sentence_arena: VdSemSentenceArenaRef<'a>,
    stmt_arena: VdSemStmtArenaRef<'a>,
    division_arena: VdSemDivisionArenaRef<'a>,
    expr_range_map: &'a VdSemExprTokenIdxRangeMap,
    phrase_range_map: &'a VdSemPhraseTokenIdxRangeMap,
    clause_range_map: &'a VdSemClauseTokenIdxRangeMap,
    sentence_range_map: &'a VdSemSentenceTokenIdxRangeMap,
    stmt_range_map: &'a VdSemStmtTokenIdxRangeMap,
    division_range_map: &'a VdSemDivisionTokenIdxRangeMap,
}
impl<'a> VdSemExprDisplayTreeBuilder<'a> {
    pub(crate) fn new(
        db: &'a salsa::Db,
        input: &'a str,
        token_storage: &'a LxTokenStorage,
        ast_arena: LxAstArenaRef<'a>,
        ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
        expr_arena: VdSemExprArenaRef<'a>,
        phrase_arena: VdSemPhraseArenaRef<'a>,
        clause_arena: VdSemClauseArenaRef<'a>,
        sentence_arena: VdSemSentenceArenaRef<'a>,
        stmt_arena: VdSemStmtArenaRef<'a>,
        division_arena: VdSemDivisionArenaRef<'a>,
        expr_range_map: &'a VdSemExprTokenIdxRangeMap,
        phrase_range_map: &'a VdSemPhraseTokenIdxRangeMap,
        clause_range_map: &'a VdSemClauseTokenIdxRangeMap,
        sentence_range_map: &'a VdSemSentenceTokenIdxRangeMap,
        stmt_range_map: &'a VdSemStmtTokenIdxRangeMap,
        division_range_map: &'a VdSemDivisionTokenIdxRangeMap,
    ) -> Self {
        Self {
            db,
            input,
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
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
        }
    }
}

impl<'a> VdSemExprDisplayTreeBuilder<'a> {
    pub fn render_expr(&self, expr: VdSemExprIdx) -> DisplayTree {
        todo!()
    }

    pub fn render_all_stmts(&self, stmts: VdSemStmtIdxRange) -> DisplayTree {
        todo!()
    }
}
