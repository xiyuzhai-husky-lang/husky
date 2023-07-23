use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitMethodFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnSynDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TraitMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitItemPath,
        decl: TraitMethodFnDecl,
    ) -> SynDefnResult<Self> {
        let TraitItemSynNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        Ok(TraitMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        ))
    }
}
