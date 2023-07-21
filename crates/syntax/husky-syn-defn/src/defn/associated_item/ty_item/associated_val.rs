use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedValNodeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node_decl: TypeAssociatedValNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedValDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedValDecl,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedValDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedValDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
