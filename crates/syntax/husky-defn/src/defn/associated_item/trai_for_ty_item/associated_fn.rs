use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedFnDefn {
    #[id]
    pub node_id: TraitForTypeItemNodeId,
    pub decl: TraitForTypeAssociatedFnDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_fn_defn(
    _db: &dyn DefnDb,
    _decl: TraitForTypeAssociatedFnDecl,
) -> TraitForTypeAssociatedFnDefn {
    todo!()
}
