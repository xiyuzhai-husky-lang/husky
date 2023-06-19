use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedValDefn {
    #[id]
    pub node_id: TraitItemNodeId,
    pub decl: TraitAssociatedValDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_val_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedValDecl,
) -> TraitAssociatedValDefn {
    todo!()
}
