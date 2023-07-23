use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FnHirDefn {
    #[id]
    pub path: FugitivePath,
    pub hir_decl: FnHirDecl,
    pub body: Option<HirExprIdx>,
    pub hir_expr_region: HirExprRegion,
}

impl FnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: FnHirDecl) -> Self {
        todo!()
        // let FugitiveHirNodeDefn::Fn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // FnHirDefn::new_inner(
        //     db,
        //     path,
        //     hir_decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.hir_expr_region(db),
        // )
    }
}
