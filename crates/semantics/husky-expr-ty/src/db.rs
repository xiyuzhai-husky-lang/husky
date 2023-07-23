use crate::*;

pub trait ExprTypeDb: salsa::DbWithJar<ExprTypeJar> + FluffyTermDb + SynDefnDb {
    fn expr_ty_region(&self, syn_expr_region: SynExprRegion) -> &ExprTypeRegion;
}

impl<Db> ExprTypeDb for Db
where
    Db: salsa::DbWithJar<ExprTypeJar> + FluffyTermDb + SynDefnDb,
{
    fn expr_ty_region(&self, syn_expr_region: SynExprRegion) -> &ExprTypeRegion {
        expr_ty_region(self, syn_expr_region)
    }
}
