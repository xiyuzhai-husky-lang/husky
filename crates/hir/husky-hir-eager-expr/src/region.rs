use husky_entity_path::region::RegionPath;

use crate::{
    var::{cvar::HirEagerComptimeVariableRegionData, rvar::HirEagerRuntimeVariableRegionData},
    *,
};

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {
    pub region_path: RegionPath,
    #[return_ref]
    pub expr_arena: HirEagerExprArena,
    #[return_ref]
    pub stmt_arena: HirEagerStmtArena,
    #[return_ref]
    pub pattern_arena: HirEagerPatternArena,
    #[return_ref]
    pub comptime_symbol_region_data: HirEagerComptimeVariableRegionData,
    #[return_ref]
    pub runtime_symbol_region_data: HirEagerRuntimeVariableRegionData,
}
