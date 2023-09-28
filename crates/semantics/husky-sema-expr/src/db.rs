use crate::*;

pub trait SemaExprDb: salsa::DbWithJar<SemaExprJar> + FluffyTermDb + SynDefnDb {
    // todo: move this somewhere else
    fn sema_expr_region(&self, syn_expr_region: SynExprRegion) -> &SemaExprRegion;
}

impl<Db> SemaExprDb for Db
where
    Db: salsa::DbWithJar<SemaExprJar> + FluffyTermDb + SynDefnDb,
{
    fn sema_expr_region(&self, syn_expr_region: SynExprRegion) -> &SemaExprRegion {
        sema_expr_region(self, syn_expr_region)
    }
}
