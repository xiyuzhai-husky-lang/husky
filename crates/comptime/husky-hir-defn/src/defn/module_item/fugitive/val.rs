use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct ValHirDefn {
    #[id]
    pub path: FugitivePath,
    pub hir_decl: ValHirDecl,
    pub body: Option<HirExprIdx>,
    pub hir_expr_region: HirEagerExprRegion,
}

impl ValHirDefn {
    pub(super) fn new(db: &dyn HirDefnDb, path: FugitivePath, hir_decl: ValHirDecl) -> Self {
        todo!()
        // let FugitiveHirNodeDefn::Val(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // Self::new_inner(
        //     db,
        //     path,
        //     hir_decl,
        //     syn_node_defn.body(db),
        //     syn_node_defn.hir_expr_region(db),
        // )
    }
}
