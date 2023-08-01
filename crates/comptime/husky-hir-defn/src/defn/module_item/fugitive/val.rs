use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct ValHirDefn {
    pub path: FugitivePath,
    pub hir_decl: ValFugitiveHirDecl,
    pub body: Option<HirExprIdx>,
    pub hir_expr_region: HirExprRegion,
}

impl ValHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: FugitivePath,
        hir_decl: ValFugitiveHirDecl,
    ) -> Self {
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
