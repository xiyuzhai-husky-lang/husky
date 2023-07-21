use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedValNodeDefn {
    #[id]
    pub node_path: TypeItemSynNodePath,
    pub node_decl: TypeAssociatedValNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedValDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedValDecl,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedValDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedValDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
