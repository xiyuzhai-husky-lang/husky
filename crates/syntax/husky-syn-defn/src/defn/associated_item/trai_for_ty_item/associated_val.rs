use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub node_decl: TraitForTypeAssociatedValNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedValDecl,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedValDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedValDecl,
    ) -> DefnResult<Self> {
        todo!()
    }
}
