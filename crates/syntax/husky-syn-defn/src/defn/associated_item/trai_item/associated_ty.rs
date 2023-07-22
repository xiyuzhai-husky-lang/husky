use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitAssociatedTypeNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeSynDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeDecl,
    pub expr_region: SynExprRegion,
}
