use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct GnHirDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: GnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl GnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, decl: GnHirDecl) -> Self {
        let FugitiveHirNodeDefn::Gn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        GnHirDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        )
    }
}
