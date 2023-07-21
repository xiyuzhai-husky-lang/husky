use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedTypeNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub node_decl: TypeAssociatedTypeNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedTypeDecl,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedTypeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedTypeDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
