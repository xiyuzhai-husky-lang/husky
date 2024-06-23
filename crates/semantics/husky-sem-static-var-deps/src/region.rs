use crate::static_var_deps::SemStaticVarDeps;
use husky_sem_expr::SemExprMap;
use vec_like::OrderedSmallVecSet;

#[salsa::tracked]
pub struct SemStaticVarDepsRegion {
    #[return_ref]
    static_var_deps_table: SemExprMap<SemStaticVarDeps>,
}
