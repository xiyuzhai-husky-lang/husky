use super::*;
use salsa::DebugWithDb;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMethodFnHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeMethodFnHirDecl,
    pub body: Option<HirEagerExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
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
        let mut builder = HirEagerExprBuilder::new(db, syn_defn.syn_expr_region(db));
        let body = syn_defn
            .body(db)
            .map(|body| body.to_hir_eager(&mut builder));
        let hir_expr_region = builder.finish();
        TypeMethodFnHirDefn::new_inner(db, path, hir_decl, body, hir_expr_region)
    }
}
