use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeMemoDecl {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_sheet: ExprSheet,
}
