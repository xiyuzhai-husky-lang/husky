use crate::*;
use husky_entity_tree::EntityTreeDb;

pub trait ExprDb: salsa::DbWithJar<SynExprJar> + EntityTreeDb {
    fn expr_range_region(&self, expr_region: SynExprRegion) -> &ExprRangeRegion;
}

impl<Db> ExprDb for Db
where
    Db: salsa::DbWithJar<SynExprJar> + EntityTreeDb,
{
    fn expr_range_region(&self, expr_region: SynExprRegion) -> &ExprRangeRegion {
        expr_range_region(self, expr_region)
    }
}
