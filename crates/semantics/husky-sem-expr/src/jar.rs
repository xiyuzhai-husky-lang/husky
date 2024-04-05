use crate::*;

pub trait SemaExprDb {
    // todo: move this somewhere else
    fn sem_expr_region(&self, syn_expr_region: SynExprRegion) -> SemaExprRegion;
}

impl SemaExprDb for ::salsa::Db {
    fn sem_expr_region(&self, syn_expr_region: SynExprRegion) -> SemaExprRegion {
        sem_expr_region(self, syn_expr_region)
    }
}

#[salsa::jar]
pub struct SemaExprJar(
    ty_ontology_path_unveil_impl_block_signature_templates,
    ty_ontology_application_unveil_impl_block_signature_templates,
    SemaExprRegion,
    crate::region::sem_expr_region,
    crate::region::sem_expr_region_eth_term_fmt_context,
    crate::helpers::analysis::sem_expr_region_requires_lazy,
    crate::helpers::range::SemaExprRangeRegion,
    crate::helpers::range::sem_expr_range_region,
);
