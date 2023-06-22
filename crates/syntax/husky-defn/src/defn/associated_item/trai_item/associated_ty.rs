use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitAssociatedTypeNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}
