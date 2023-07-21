use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node_decl: TraitForTypeAssociatedValNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedValDecl,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedValDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedValDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
