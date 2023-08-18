use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedFnHirDecl,
    pub body: Option<HirEagerExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
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
        let mut builder = HirEagerExprBuilder::new(db, syn_defn.syn_expr_region(db));
        let body = syn_defn
            .body(db)
            .map(|body| body.to_hir_eager(&mut builder));
        let hir_expr_region = builder.finish();
        TypeAssociatedFnHirDefn::new_inner(db, path, hir_decl, body, hir_expr_region)
    }
}
