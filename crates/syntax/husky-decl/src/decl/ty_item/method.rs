use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeMethodDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
