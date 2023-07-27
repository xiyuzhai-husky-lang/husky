use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedFnHirDecl,
    pub body: Option<HirExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        hir_decl: TypeAssociatedFnHirDecl,
    ) -> TypeAssociatedFnHirDefn {
        todo!()
        // let TypeItemHirNodeDefn::AssociatedFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Ok(TypeAssociatedFnHirDefn::new_inner(
        //     db,
        //     path,
        //     hir_decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.hir_expr_region(db),
        // ))
    }
}
