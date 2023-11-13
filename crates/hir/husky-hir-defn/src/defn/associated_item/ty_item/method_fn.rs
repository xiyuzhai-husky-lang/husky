use super::*;


#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMethodFnHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeMethodFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl TypeMethodFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        hir_decl: TypeMethodFnHirDecl,
    ) -> Self {
        let Ok(TypeItemSynDefn::MethodFn(syn_defn)) = path.syn_defn(db) else {
            unreachable!("hir stage no error")
        };
        TypeMethodFnHirDefn::new_inner(
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
