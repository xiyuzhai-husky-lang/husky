use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: FnFugitiveHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl FnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: FnFugitiveHirDecl) -> Self {
        let Ok(FugitiveSynDefn::Fn(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        FnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            build_eager_body(syn_defn.body_with_syn_expr_region(db), db),
        )
    }
}
