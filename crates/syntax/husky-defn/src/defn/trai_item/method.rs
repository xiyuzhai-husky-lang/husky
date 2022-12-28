use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TraitMethodDefn {
    #[id]
    pub entity_path: EntityPath,
    pub decl: TraitMethodDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
