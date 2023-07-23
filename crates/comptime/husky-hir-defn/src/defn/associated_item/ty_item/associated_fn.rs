use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedFnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl TypeAssociatedFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedFnHirDecl,
    ) -> TypeAssociatedFnHirDefn {
        todo!()
        // let TypeItemHirNodeDefn::AssociatedFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Ok(TypeAssociatedFnHirDefn::new_inner(
        //     db,
        //     path,
        //     decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.expr_region(db),
        // ))
    }
}
