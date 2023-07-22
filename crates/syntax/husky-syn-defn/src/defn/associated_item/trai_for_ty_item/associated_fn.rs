use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeAssociatedFnNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedFnDecl,
    pub expr_region: SynExprRegion,
}
