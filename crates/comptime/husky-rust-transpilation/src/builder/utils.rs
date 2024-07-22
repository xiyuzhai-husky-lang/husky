use husky_entity_path::region::RegionPath;
use husky_place::place::EthPlace;

use super::*;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b, HirEagerExprRegion> {
    pub(crate) fn show_place_info(&self, place: EthPlace) -> String {
        let db = self.db;
        let place_registry = self
            .hir_eager_expr_region()
            .sem_expr_region(db)
            .data(db)
            .place_registry();
        place.show_info(db, place_registry)
    }

    pub(crate) fn is_defn_region(&self, ident_str: &str) -> bool {
        let db = self.db;
        match self.hir_eager_expr_region().region_path(db) {
            RegionPath::ItemDefn(item_path) => match item_path.ident(db) {
                Some(ident) => ident.data(db) == ident_str,
                None => false,
            },
            _ => false,
        }
    }
}
