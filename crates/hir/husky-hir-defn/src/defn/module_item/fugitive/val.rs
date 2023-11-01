use super::*;
use husky_hir_expr::helpers::hir_body_with_expr_region;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct ValHirDefn {
    pub path: FugitivePath,
    pub hir_decl: ValFugitiveHirDecl,
    pub body_with_hir_expr_region: Option<(HirExprIdx, HirExprRegion)>,
}

impl ValHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: FugitivePath,
        hir_decl: ValFugitiveHirDecl,
    ) -> Self {
        let Ok(FugitiveSynDefn::Val(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        Self::new_inner(
            db,
            path,
            hir_decl,
            hir_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        self.body_with_hir_expr_region(db).map(|v| v.1)
    }
}
