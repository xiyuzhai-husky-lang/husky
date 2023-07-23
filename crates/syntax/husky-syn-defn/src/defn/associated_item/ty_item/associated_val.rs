use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedValSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeAssociatedValNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedValSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedValDecl,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedValSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedValDecl,
    ) -> SynDefnResult<Self> {
        todo!()
    }
}
