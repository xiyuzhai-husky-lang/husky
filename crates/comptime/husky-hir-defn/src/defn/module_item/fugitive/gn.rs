use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct GnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: GnFugitiveHirDecl,
    pub body: Option<HirEagerExprIdx>,
    pub hir_expr_region: HirLazyExprRegion,
}

impl GnHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: GnFugitiveHirDecl) -> Self {
        todo!()
        // let FugitiveHirNodeDefn::Gn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // GnHirDefn::new_inner(
        //     db,
        //     path,
        //     hir_decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.hir_expr_region(db),
        // )
    }
}
