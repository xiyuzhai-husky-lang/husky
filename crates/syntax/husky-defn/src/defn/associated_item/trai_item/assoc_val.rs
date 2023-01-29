use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TraitAssociatedValueDecl,
}
