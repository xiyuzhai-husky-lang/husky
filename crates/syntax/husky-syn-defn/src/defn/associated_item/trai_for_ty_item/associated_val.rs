use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeAssociatedValSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedValSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitForTypeAssociatedValSynDefn {
    pub(super) fn new(
        _db: &::salsa::Db,
        _path: TraitForTypeItemPath,
        _decl: TraitForTypeAssociatedValSynDecl,
    ) -> SynDefnResult<Self> {
        todo!()
    }
}
