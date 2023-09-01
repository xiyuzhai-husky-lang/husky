use crate::{db::*, *};

#[salsa::tracked(db = HirLazyExprDb, jar = HirLazyExprJar)]
pub struct HirLazyExprRegion {
    #[return_ref]
    pub expr_arena: HirLazyExprArena,
    #[return_ref]
    pub stmt_arena: HirLazyStmtArena,
    #[return_ref]
    pub pattern_expr_arena: HirLazyPatternExprArena,
}
