use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitMethodFnNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub node_decl: TraitMethodFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TraitMethodFnDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitItemPath,
        decl: TraitMethodFnDecl,
    ) -> DefnResult<Self> {
        let TraitItemSynNodeDefn::MethodFn(node_defn) = path.syn_node_path(db).node_defn(db) else {
            unreachable!()
        };
        Ok(TraitMethodFnDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        ))
    }
}
