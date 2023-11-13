use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitMethodFnHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitMethodFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl TraitMethodFnHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TraitItemPath,
        _hir_decl: TraitMethodFnHirDecl,
    ) -> Self {
        todo!()
        // let TraitItemHirNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Ok(TraitMethodFnHirDefn::new_inner(
        //     db,
        //     path,
        //     hir_decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.hir_expr_region(db),
        // ))
    }
}
