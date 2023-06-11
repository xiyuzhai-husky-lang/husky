use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedFnDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TraitAssociatedFnDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_fn_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedFnDecl,
) -> TraitAssociatedFnDefn {
    todo!()
}
