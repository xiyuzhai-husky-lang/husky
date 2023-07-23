use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnHirDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeMethodFnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl TraitForTypeMethodFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeMethodFnHirDecl,
    ) -> TraitForTypeMethodFnHirDefn {
        todo!()
        // let TraitForTypeItemSynDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // TraitForTypeMethodFnHirDefn::new_inner(
        //     db,
        //     path,
        //     decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.expr_region(db),
        // )
    }
}
