use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TraitAssociatedValueDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_value_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedValueDecl,
) -> TraitAssociatedValueDefn {
    todo!()
}
