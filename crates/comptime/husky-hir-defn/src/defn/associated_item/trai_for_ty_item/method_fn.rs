use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeMethodFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
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
        TraitForTypeMethodFnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }
}
