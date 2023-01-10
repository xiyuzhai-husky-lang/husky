use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAsTraitAssociatedFunctionDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_sheet: ExprSheet,
}
