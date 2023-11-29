use husky_entity_path::region::RegionPath;

use crate::{
    symbol::{
        comptime_symbol::HirEagerComptimeSymbolRegionData,
        runtime_symbol::HirEagerRuntimeSymbolRegionData,
    },
    *,
};

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {
    pub path: RegionPath,
    #[return_ref]
    pub expr_arena: HirEagerExprArena,
    #[return_ref]
    pub stmt_arena: HirEagerStmtArena,
    #[return_ref]
    pub pattern_expr_arena: HirEagerPatternExprArena,
    #[return_ref]
    pub comptime_symbol_region_data: HirEagerComptimeSymbolRegionData,
    #[return_ref]
    pub runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData,
}
