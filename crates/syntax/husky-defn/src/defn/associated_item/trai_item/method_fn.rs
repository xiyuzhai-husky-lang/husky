use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TraitMethodFnNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitMethodFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TraitMethodFnDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TraitItemPath,
        decl: TraitMethodFnDecl,
    ) -> DefnResult<Self> {
        let TraitItemNodeDefn::MethodFn(node_defn) = path.node_path(db).node_defn(db) else {
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
