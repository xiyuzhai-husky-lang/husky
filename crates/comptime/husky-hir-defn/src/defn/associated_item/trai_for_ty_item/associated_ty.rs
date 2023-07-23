use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeHirDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedTypeHirDecl,
    pub hir_expr_region: HirExprRegion,
}

impl TraitForTypeAssociatedTypeHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeAssociatedTypeHirDecl,
    ) -> Self {
        todo!()
        // let TraitForTypeItemHirNodeDefn::AssociatedType(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
        //     unreachable!()
        // };
        // TraitForTypeAssociatedTypeHirDefn::new_inner(db, path, hir_decl, syn_node_defn.hir_expr_region(db))
    }
}
