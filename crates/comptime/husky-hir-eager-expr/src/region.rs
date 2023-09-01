use crate::*;

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {
    #[return_ref]
    pub expr_arena: HirEagerExprArena,
    #[return_ref]
    pub stmt_arena: HirEagerStmtArena,
    #[return_ref]
    pub pattern_expr_arena: HirEagerPatternExprArena,
}
