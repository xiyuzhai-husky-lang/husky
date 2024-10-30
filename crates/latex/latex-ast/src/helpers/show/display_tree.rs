use crate::{
    ast::{math::LxMathAstIdx, rose::LxRoseAstIdx, LxAstArenaRef, LxAstIdx, LxAstIdxRange},
    range::LxAstTokenIdxRangeMap,
};
use ptree::TreeBuilder;

struct LxAstDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    input: &'a str,
    ast_arena: LxAstArenaRef<'a>,
    ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    tree_builder: TreeBuilder,
}

/// # construction
impl<'a> LxAstDisplayTreeBuilder<'a> {
    fn new(
        db: &'a salsa::Db,
        input: &'a str,
        ast_arena: LxAstArenaRef<'a>,
        ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    ) -> Self {
        Self {
            db,
            input,
            ast_arena,
            ast_token_idx_range_map,
            tree_builder: TreeBuilder::new(input.to_string()),
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
        todo!()
    }

    fn render_rose_ast(&mut self, ast: LxRoseAstIdx) {
        todo!()
    }
}
