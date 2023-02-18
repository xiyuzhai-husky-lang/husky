use crate::*;
use husky_entity_tree::EntityTreeDb;

pub trait ExprDb: salsa::DbWithJar<ExprJar> + EntityTreeDb {
    fn expr_range_region(&self, expr_region: ExprRegion) -> &ExprRangeRegion;
}

impl<Db> ExprDb for Db
where
    Db: salsa::DbWithJar<ExprJar> + EntityTreeDb,
{
    fn expr_range_region(&self, expr_region: ExprRegion) -> &ExprRangeRegion {
        expr_range_region(self, expr_region)
    }
}
