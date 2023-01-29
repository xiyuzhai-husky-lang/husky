use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitMethodDefn {
    #[id]
    pub entity_path: EntityPath,
    pub decl: TraitMethodDecl,
    pub expr_region: ExprRegion,
}
