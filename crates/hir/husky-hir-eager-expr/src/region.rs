use crate::{
    variable::{
        comptime::HirEagerComptimeVariableRegionData, runtime::HirEagerRuntimeVariableRegionData,
    },
    *,
};
use husky_entity_path::region::RegionPath;
use husky_hir_ty::HirType;

/// this is interned on purpose
#[salsa::interned(db = HirEagerExprDb, jar = HirEagerExprJar)]
pub struct HirEagerExprRegion {
    pub region_path: RegionPath,
    pub self_value_ty: Option<HirType>,
    #[return_ref]
    pub expr_arena: HirEagerExprArena,
    #[return_ref]
    pub stmt_arena: HirEagerStmtArena,
    #[return_ref]
    pub pattern_arena: HirEagerPatternArena,
    #[return_ref]
    pub comptime_variable_region_data: HirEagerComptimeVariableRegionData,
    #[return_ref]
    pub runtime_variable_region_data: HirEagerRuntimeVariableRegionData,
}
