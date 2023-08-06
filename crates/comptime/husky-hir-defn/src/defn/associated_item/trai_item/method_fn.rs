use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitMethodFnHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitMethodFnHirDecl,
    pub body: Option<HirEagerExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitMethodFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitItemPath,
        hir_decl: TraitMethodFnHirDecl,
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
