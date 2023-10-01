use husky_hir_lazy_expr::helpers::build_lazy_body;

use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct GnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: GnFugitiveHirDecl,
    pub lazy_body_with_hir_lazy_expr_region: Option<(HirLazyExprIdx, HirLazyExprRegion)>,
}

impl GnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: GnFugitiveHirDecl) -> Self {
        let Ok(FugitiveSynDefn::FunctionGn(syn_defn)) = path.syn_defn(db) else {
            unreachable!("hir stage no error")
        };
        GnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            build_lazy_body(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_lazy_expr_region(self, db: &dyn HirDefnDb) -> Option<HirLazyExprRegion> {
        Some(self.lazy_body_with_hir_lazy_expr_region(db)?.1)
    }
}
