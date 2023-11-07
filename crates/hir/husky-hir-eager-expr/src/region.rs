use crate::*;
use husky_sema_expr::{SemaExprMap, SemaExprRegion};

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {
    #[return_ref]
    pub hir_eager_expr_arena: HirEagerExprArena,
    #[return_ref]
    pub hir_eager_stmt_arena: HirEagerStmtArena,
    #[return_ref]
    pub hir_eager_pattern_expr_arena: HirEagerPatternExprArena,
    #[return_ref]
    pub hir_eager_variable_region: HirEagerVariableRegion,
}
