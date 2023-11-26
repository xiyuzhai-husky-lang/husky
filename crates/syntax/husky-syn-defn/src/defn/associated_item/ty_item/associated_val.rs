use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAssociatedValSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeAssociatedValSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedValSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedValSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeAssociatedValSynDefn {
    pub(super) fn new(
        _db: &::salsa::Db,
        _path: TypeItemPath,
        _decl: TypeAssociatedValSynDecl,
    ) -> SynDefnResult<Self> {
        todo!()
    }
}
