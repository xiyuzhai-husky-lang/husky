use crate::{
    ast::{math::LxMathAstIdx, rose::LxRoseAstIdx, LxAstArenaRef, LxAstIdx, LxAstIdxRange},
    range::LxAstTokenIdxRangeMap,
};
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
    fn render_asts(&mut self, asts: LxAstIdxRange) {
        match asts {
            LxAstIdxRange::Math(range) => {
                for ast in range {
                    self.render_math_ast(ast);
                }
            }
            LxAstIdxRange::Rose(range) => {
                for ast in range {
                    self.render_rose_ast(ast);
                }
            }
        }
    }

    fn render_math_ast(&mut self, ast: LxMathAstIdx) {
        let ast_token_idx_range = self.ast_token_idx_range_map[ast];
        let (start, end) = self
            .token_storage
            .math_token_idx_range_offset_range(ast_token_idx_range);
        // self.tree_builder
        //     .add_empty_child(text)
        //     .add_node(self.input[start..end].to_string());
        todo!()
    }

    fn render_rose_ast(&mut self, ast: LxRoseAstIdx) {
        todo!()
    }
}
