use crate::*;

pub trait SemaExprDb: salsa::DbWithJar<SemaExprJar> + FluffyTermDb + SynDefnDb {
    // todo: move this somewhere else
    fn expr_ty_region(&self, syn_expr_region: SynExprRegion) -> &SemaExprRegion;
}

impl<Db> SemaExprDb for Db
where
    Db: salsa::DbWithJar<SemaExprJar> + FluffyTermDb + SynDefnDb,
{
    fn expr_ty_region(&self, syn_expr_region: SynExprRegion) -> &SemaExprRegion {
        expr_ty_region(self, syn_expr_region)
    }
}
