use crate::{
    builder::VdSynExprBuilder,
    expr::{VdSynExprArenaRef, VdSynExprIdx, VdSynExprIdxRange},
    range::{VdSynExprRange, VdSynExprRangeMap},
};
use husky_tree_utils::display::DisplayTree;
use latex_ast::{
    ast::{
        math::{LxMathAstIdx, LxMathAstIdxRange},
        rose::{LxRoseAstIdx, LxRoseAstIdxRange},
        LxAstArenaRef, LxAstIdx, LxAstIdxRange,
    },
    range::LxAstTokenIdxRangeMap,
};
use latex_token::storage::LxTokenStorage;

pub struct VdSynExprDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    input: &'a str,
    token_storage: &'a LxTokenStorage,
    ast_arena: LxAstArenaRef<'a>,
    ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    expr_arena: VdSynExprArenaRef<'a>,
    expr_range_map: &'a VdSynExprRangeMap<'a>,
}

/// # construction
impl<'a> VdSynExprDisplayTreeBuilder<'a> {
    pub fn new(
        db: &'a salsa::Db,
        input: &'a str,
        token_storage: &'a LxTokenStorage,
        ast_arena: LxAstArenaRef<'a>,
        ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
        expr_arena: VdSynExprArenaRef<'a>,
        expr_range_map: &'a VdSynExprRangeMap<'a>,
    ) -> Self {
        Self {
            db,
            input,
            token_storage,
            ast_arena,
            ast_token_idx_range_map,
            expr_arena,
            expr_range_map,
        }
    }
}

/// # actions
impl<'a> VdSynExprDisplayTreeBuilder<'a> {
    pub fn render_all(&self, exprs: VdSynExprIdxRange) -> DisplayTree {
        // TODO: maybe just use the range covered by these exprs
        DisplayTree::new(self.input.to_string(), self.render_exprs(exprs))
    }

    pub fn render_exprs(&self, exprs: impl IntoIterator<Item = VdSynExprIdx>) -> Vec<DisplayTree> {
        exprs
            .into_iter()
            .map(|expr| self.render_expr(expr))
            .collect()
    }

    pub fn render_expr(&self, expr: VdSynExprIdx) -> DisplayTree {
        let expr_range = self.expr_range_map[expr];
        let (start, end) = match expr_range {
            VdSynExprRange::Ast(ast) => self.ast_offset_range(ast),
            VdSynExprRange::Asts(asts) => todo!(),
        };
        let value = self.input[(start..end)].to_string();
        DisplayTree::new(value, self.render_exprs(self.expr_arena[expr].children()))
    }

    fn ast_offset_range(&self, ast: LxAstIdx) -> (usize, usize) {
        match ast {
            LxAstIdx::Math(ast) => self.math_ast_offset_range(ast),
            LxAstIdx::Rose(ast) => self.rose_ast_offset_range(ast),
        }
    }

    fn math_ast_offset_range(&self, ast: LxMathAstIdx) -> (usize, usize) {
        let range = self.ast_token_idx_range_map[ast];
        self.token_storage.math_token_idx_range_offset_range(range)
    }

    fn rose_ast_offset_range(&self, ast: LxRoseAstIdx) -> (usize, usize) {
        let range = self.ast_token_idx_range_map[ast];
        self.token_storage.rose_token_idx_range_offset_range(range)
    }

    fn asts_offset_range(&self, asts: LxAstIdxRange) -> (usize, usize) {
        match asts {
            LxAstIdxRange::Math(asts) => self.math_asts_offset_range(asts),
            LxAstIdxRange::Rose(asts) => self.rose_asts_offset_range(asts),
        }
    }

    fn math_asts_offset_range(&self, asts: LxMathAstIdxRange) -> (usize, usize) {
        let first = asts.start();
        let Some(last) = asts.last() else { todo!() };
        (
            self.math_ast_offset_range(first).0,
            self.math_ast_offset_range(last).1,
        )
    }

    fn rose_asts_offset_range(&self, asts: LxRoseAstIdxRange) -> (usize, usize) {
        let first = asts.start();
        let Some(last) = asts.last() else { todo!() };
        (
            self.rose_ast_offset_range(first).0,
            self.rose_ast_offset_range(last).1,
        )
    }
}

// impl<'db> VdSynExprBuilder<'db> {
//     pub fn display_tree(&self) -> DisplayTree {
//         let display_tree_builder = LxAstDisplayTreeBuilder::new(
//             self.db,
//             self.input,
//             self.token_storage,
//             self.ast_arena,
//             self.ast_token_idx_range_map,
//         );
//         display_tree_builder.render_all(self.expr_range_map.keys())
//     }
// }
