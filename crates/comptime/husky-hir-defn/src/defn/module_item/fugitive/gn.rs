use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct GnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: GnFugitiveHirDecl,
    pub lazy_body_with_hir_lazy_expr_region: Option<(HirLazyExprIdx, HirLazyExprRegion)>,
}

impl GnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: GnFugitiveHirDecl) -> Self {
        let Ok(FugitiveSynDefn::Gn(syn_defn)) = path.syn_defn(db) else {
            unreachable!("hir stage no error")
        };
        let mut builder = HirLazyExprBuilder::new(db, syn_defn.syn_expr_region(db));
        let body = syn_defn.body(db).map(|body| body.to_hir_lazy(&mut builder));
        let hir_expr_region = builder.finish();
        GnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            build_eager_body(syn_defn.body_with_syn_expr_region(db), db),
        )
    }
}
