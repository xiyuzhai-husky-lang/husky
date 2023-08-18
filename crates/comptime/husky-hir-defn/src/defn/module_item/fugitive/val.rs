use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct ValHirDefn {
    pub path: FugitivePath,
    pub hir_decl: ValFugitiveHirDecl,
    pub body: Option<HirExprIdx>,
    pub hir_expr_region: HirExprRegion,
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
        let mut builder = HirExprBuilder::new(db, syn_defn.syn_expr_region(db));
        let body = syn_defn.body(db).map(|body| body.to_hir(&mut builder));
        let hir_expr_region = builder.finish();
        Self::new_inner(db, path, hir_decl, body, hir_expr_region)
    }
}
