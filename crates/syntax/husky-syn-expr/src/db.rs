use crate::*;
use husky_entity_syn_tree::EntitySynTreeDb;

pub trait SynExprDb: salsa::DbWithJar<SynExprJar> + EntitySynTreeDb {
    fn expr_range_region(&self, expr_region: SynExprRegion) -> &SynExprRangeRegion;
}

impl<Db> SynExprDb for Db
where
    Db: salsa::DbWithJar<SynExprJar> + EntitySynTreeDb,
{
    fn expr_range_region(&self, expr_region: SynExprRegion) -> &SynExprRangeRegion {
        syn_expr_range_region(self, expr_region)
    }
}
