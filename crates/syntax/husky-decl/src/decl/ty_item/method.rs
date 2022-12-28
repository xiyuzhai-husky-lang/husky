use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeMethodDecl {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
