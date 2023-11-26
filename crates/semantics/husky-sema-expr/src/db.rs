use crate::*;

pub trait SemaExprDb: salsa::DbWithJar<SemaExprJar> + FluffyTermDb + SynDefnDb {
    // todo: move this somewhere else
    fn sema_expr_region(&self, syn_expr_region: SynExprRegion) -> SemaExprRegion;
}

impl SemaExprDb for Db
where
    Db: salsa::DbWithJar<SemaExprJar> + FluffyTermDb + SynDefnDb,
{
    fn sema_expr_region(&self, syn_expr_region: SynExprRegion) -> SemaExprRegion {
        sema_expr_region(self, syn_expr_region)
    }
}

#[salsa::jar(db = SemaExprDb)]
pub struct SemaExprJar(
    ty_ontology_path_unveil_impl_block_signature_templates,
    ty_ontology_application_unveil_impl_block_signature_templates,
    SemaExprRegion,
    sema_expr_region,
    crate::helpers::analysis::sema_expr_region_contains_gn,
    crate::helpers::range::SemaExprRangeRegion,
    crate::helpers::range::sema_expr_range_region,
);
