use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitAssociatedTypeNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}
