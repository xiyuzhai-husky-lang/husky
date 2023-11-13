use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeAssociatedTypeSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedTypeSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeAssociatedTypeSynDefn {
    pub(super) fn new(
        _db: &dyn SynDefnDb,
        _path: TypeItemPath,
        _decl: TypeAssociatedTypeSynDecl,
    ) -> SynDefnResult<Self> {
        todo!()
    }
}
