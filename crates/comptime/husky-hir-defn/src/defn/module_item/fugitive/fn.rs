use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: FnFugitiveHirDecl,
    pub body: Option<HirEagerExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
}

impl FnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: FnFugitiveHirDecl) -> Self {
        let FugitiveSynDefn::Fn(syn_defn) = path.syn_defn(db).expect("hir stage no error") else {
            unreachable!()
        };
        let mut builder = HirEagerExprBuilder::new(db, syn_defn.syn_expr_region(db));
        let body = syn_defn.body(db).map(|body| builder.new_expr(body));
        let hir_expr_region = builder.finish();
        FnHirDefn::new_inner(db, path, hir_decl, body, hir_expr_region)
    }
}
