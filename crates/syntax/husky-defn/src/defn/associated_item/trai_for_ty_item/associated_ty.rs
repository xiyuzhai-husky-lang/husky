use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node_decl: TraitForTypeAssociatedTypeNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedTypeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedTypeDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
