use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMemoizedFieldHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl TypeMemoizedFieldHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        decl: TypeMemoizedFieldHirDecl,
    ) -> TypeMemoizedFieldHirDefn {
        todo!()
        // let TypeItemHirNodeDefn::MemoizedField(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // TypeMemoizedFieldHirDefn::new_inner(
        //     db,
        //     path,
        //     decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.expr_region(db),
        // )
    }
}
