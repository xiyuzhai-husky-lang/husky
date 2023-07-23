use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FnHirDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: FnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl FnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, decl: FnHirDecl) -> Self {
        let FugitiveHirNodeDefn::Fn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        FnHirDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        )
    }
}
