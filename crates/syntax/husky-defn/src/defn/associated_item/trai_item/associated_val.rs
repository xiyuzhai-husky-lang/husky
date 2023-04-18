use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedValDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TraitAssociatedValueDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_val_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedValueDecl,
) -> TraitAssociatedValDefn {
    todo!()
}
