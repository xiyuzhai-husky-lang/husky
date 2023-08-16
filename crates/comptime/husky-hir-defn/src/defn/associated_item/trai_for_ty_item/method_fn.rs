use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeMethodFnHirDecl,
    pub body: Option<HirEagerExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitForTypeMethodFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeMethodFnHirDecl,
    ) -> TraitForTypeMethodFnHirDefn {
        let TraitForTypeItemSynDefn::MethodFn(syn_defn) =
            path.syn_defn(db).expect("hir stage no error")
        else {
            unreachable!()
        };
        let mut builder = HirEagerExprBuilder::new(db, syn_defn.syn_expr_region(db));
        let body = syn_defn
            .body(db)
            .map(|body| body.to_hir_eager(&mut builder));
        let hir_expr_region = builder.finish();
        TraitForTypeMethodFnHirDefn::new_inner(db, path, hir_decl, body, hir_expr_region)
    }
}
