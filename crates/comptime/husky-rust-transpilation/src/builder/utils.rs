use husky_place::place::EthPlace;

use super::*;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b, HirEagerExprRegion> {
    pub(crate) fn show_place_info(&self, place: EthPlace) -> String {
        let db = self.db;
        let place_registry = self
            .hir_eager_expr_region()
            .sema_expr_region(db)
            .data(db)
            .place_registry();
        place.show_info(db, place_registry)
    }
}
