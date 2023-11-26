use crate::*;
use husky_entity_syn_tree::EntitySynTreeDb;

pub trait SynExprDb {
    fn expr_range_region(&self, expr_region: SynExprRegion) -> &SynExprRangeRegion;
}

impl SynExprDb for ::salsa::Db {
    fn expr_range_region(&self, expr_region: SynExprRegion) -> &SynExprRangeRegion {
        syn_expr_range_region(self, expr_region)
    }
}
