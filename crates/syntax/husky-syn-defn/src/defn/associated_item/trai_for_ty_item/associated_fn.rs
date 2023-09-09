use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeAssociatedFnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedFnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}
