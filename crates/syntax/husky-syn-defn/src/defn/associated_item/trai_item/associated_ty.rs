use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TraitItemSynNodePath,
    pub node_decl: TraitAssociatedTypeNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeDecl,
    pub expr_region: SynExprRegion,
}
