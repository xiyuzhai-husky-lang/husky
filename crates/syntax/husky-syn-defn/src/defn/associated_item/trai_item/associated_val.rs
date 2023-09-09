use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedValSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitAssociatedValSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValSynDefn {
    #[id]
    pub syn_node_path: TraitItemPath,
    pub decl: TraitAssociatedValSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}
