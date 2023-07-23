use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeAssociatedTypeSynNodeDecl,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedTypeSynDecl,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedTypeSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedTypeSynDecl,
    ) -> SynDefnResult<Self> {
        todo!()
    }
}
