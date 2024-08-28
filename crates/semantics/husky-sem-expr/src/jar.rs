use crate::*;

pub trait SemExprDb {
    // todo: move this somewhere else
    fn sem_expr_region(&self, syn_expr_region: SynExprRegion) -> SemExprRegion;
}

impl SemExprDb for ::salsa::Db {
    fn sem_expr_region(&self, syn_expr_region: SynExprRegion) -> SemExprRegion {
        sem_expr_region(self, syn_expr_region)
    }
}

#[salsa::jar]
pub struct SemExprJar(
    SemExprRegion,
    crate::region::sem_expr_region,
    crate::htmx::sem_expr_htmx_region,
    crate::helpers::analysis::sem_expr_region_requires_lazy,
    crate::helpers::range::SemExprRangeRegion,
    crate::helpers::range::sem_expr_range_region,
);
