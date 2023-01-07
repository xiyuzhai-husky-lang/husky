use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeMemoDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_sheet: AssociatedItemDeclExprSheet,
}
