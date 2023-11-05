use husky_hir_lazy_expr::helpers::hir_lazy_body_with_expr_region;

use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FunctionGnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: FunctionGnFugitiveHirDecl,
    pub lazy_body_with_hir_lazy_expr_region: Option<(HirLazyExprIdx, HirLazyExprRegion)>,
}

impl FunctionGnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: FugitivePath,
        hir_decl: FunctionGnFugitiveHirDecl,
    ) -> Self {
        let Ok(FugitiveSynDefn::FunctionGn(syn_defn)) = path.syn_defn(db) else {
            unreachable!("hir stage no error")
        };
        FunctionGnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_lazy_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_lazy_expr_region(self, db: &dyn HirDefnDb) -> Option<HirLazyExprRegion> {
        Some(self.lazy_body_with_hir_lazy_expr_region(db)?.1)
    }
}
