use super::*;
use salsa::DebugWithDb;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMethodFnHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMethodFnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl TypeMethodFnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: TypeItemPath, decl: TypeMethodFnHirDecl) -> Self {
        todo!()
        // let TypeItemHirNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Ok(TypeMethodFnHirDefn::new_inner(
        //     db,
        //     path,
        //     decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.expr_region(db),
        // ))
    }
}
