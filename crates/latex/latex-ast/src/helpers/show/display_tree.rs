use crate::{
    ast::{
        math::{LxMathAstIdx, LxMathAstIdxRange},
        rose::LxRoseAstIdx,
        LxAstArenaRef, LxAstIdx, LxAstIdxRange,
    },
    range::LxAstTokenIdxRangeMap,
};
use husky_tree_utils::display::DisplayTree;
use latex_token::storage::LxTokenStorage;

struct LxAstDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    input: &'a str,
    ast_arena: LxAstArenaRef<'a>,
    ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    token_storage: &'a LxTokenStorage,
}

/// # construction
impl<'a> LxAstDisplayTreeBuilder<'a> {
    fn new(
        db: &'a salsa::Db,
        input: &'a str,
        token_storage: &'a LxTokenStorage,
        ast_arena: LxAstArenaRef<'a>,
        ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    ) -> Self {
        Self {
            db,
            input,
            ast_arena,
            ast_token_idx_range_map,
            token_storage,
        }
    }
}

/// # actions
impl<'a> LxAstDisplayTreeBuilder<'a> {
    fn render_asts(&self, asts: LxAstIdxRange) -> Vec<DisplayTree> {
        match asts {
            LxAstIdxRange::Math(range) => self.render_math_asts(range),
            LxAstIdxRange::Rose(range) => self.render_rose_asts(range),
        }
    }

    fn render_math_asts(&self, asts: impl IntoIterator<Item = LxMathAstIdx>) -> Vec<DisplayTree> {
        asts.into_iter()
            .map(|ast| self.render_math_ast(ast))
            .collect()
    }

    fn render_rose_asts(&self, asts: impl IntoIterator<Item = LxRoseAstIdx>) -> Vec<DisplayTree> {
        asts.into_iter()
            .map(|ast| self.render_rose_ast(ast))
            .collect()
    }

    fn render_math_ast(&self, ast: LxMathAstIdx) -> DisplayTree {
        let ast_token_idx_range = self.ast_token_idx_range_map[ast];
        let (start, end) = self
            .token_storage
            .math_token_idx_range_offset_range(ast_token_idx_range);
        let value = self.input[start..end].to_string();
        DisplayTree::new(
            value,
            self.render_math_asts(self.ast_arena.math()[ast].children()),
        )
    }

    fn render_rose_ast(&self, ast: LxRoseAstIdx) -> DisplayTree {
        todo!()
    }
}
