use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemSynNodePath,
    pub node_decl: TraitForTypeAssociatedFnNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedFnDecl,
    pub expr_region: SynExprRegion,
}
