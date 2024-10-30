use crate::{
    ast::{LxAstArenaRef, LxAstIdx, LxAstIdxRange},
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
        for ast in asts {
            self.render_ast(ast);
        }
    }

    fn render_ast(&mut self, ast: LxAstIdx) {
        todo!()
    }
}
