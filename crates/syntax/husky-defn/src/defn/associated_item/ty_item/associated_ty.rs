use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node_decl: TypeAssociatedTypeNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedTypeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedTypeDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
