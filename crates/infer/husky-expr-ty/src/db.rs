use crate::*;

pub trait ExprTermDb: salsa::DbWithJar<ExprTypeJar> + TermDb + DefnDb {
    fn expr_ty_region(&self, expr_region: ExprRegion) -> &ExprTypeRegion;
}

impl<Db> ExprTermDb for Db
where
    Db: salsa::DbWithJar<ExprTypeJar> + TermDb + DefnDb,
{
    fn expr_ty_region(&self, expr_region: ExprRegion) -> &ExprTypeRegion {
        expr_ty_region(self, expr_region)
    }
}
