use crate::static_mut_deps::SemValueStaticMutDeps;
use husky_sem_expr::SemExprMap;
use vec_like::OrderedSmallVecSet;

#[salsa::tracked]
pub struct SemStaticMutDepsRegion {
    #[return_ref]
    static_mut_deps_table: SemExprMap<SemValueStaticMutDeps>,
}
