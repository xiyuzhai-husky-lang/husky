use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl TypeAssociatedFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        hir_decl: TypeAssociatedFnHirDecl,
    ) -> TypeAssociatedFnHirDefn {
        let Ok(TypeItemSynDefn::AssociatedFn(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        TypeAssociatedFnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &dyn HirDefnDb) -> Option<HirEagerExprRegion> {
        Some(self.eager_body_with_hir_eager_expr_region(db)?.1)
    }
}
