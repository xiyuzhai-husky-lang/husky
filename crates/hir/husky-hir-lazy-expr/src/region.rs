use crate::{db::*, variable::HirLazyVariableRegion, *};

/// this is interned on purpose
#[salsa::interned(db = HirLazyExprDb, jar = HirLazyExprJar)]
pub struct HirLazyExprRegion {
    #[return_ref]
    pub hir_lazy_expr_arena: HirLazyExprArena,
    #[return_ref]
    pub hir_lazy_stmt_arena: HirLazyStmtArena,
    #[return_ref]
    pub hir_lazy_pattern_expr_arena: HirLazyPatternExprArena,
    #[return_ref]
    pub hir_lazy_variable_region: HirLazyVariableRegion,
}
