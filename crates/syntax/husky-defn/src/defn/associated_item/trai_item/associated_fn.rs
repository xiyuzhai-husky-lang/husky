use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedFunctionDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TraitAssociatedFunctionDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_function_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedFunctionDecl,
) -> TraitAssociatedFunctionDefn {
    todo!()
}
