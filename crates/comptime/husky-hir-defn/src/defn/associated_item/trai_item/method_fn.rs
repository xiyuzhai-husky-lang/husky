use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitMethodFnHirDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl TraitMethodFnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: TraitItemPath, decl: TraitMethodFnHirDecl) -> Self {
        todo!()
        // let TraitItemHirNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Ok(TraitMethodFnHirDefn::new_inner(
        //     db,
        //     path,
        //     decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.expr_region(db),
        // ))
    }
}
