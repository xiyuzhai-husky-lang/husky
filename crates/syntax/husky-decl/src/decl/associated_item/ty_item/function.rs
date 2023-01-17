use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAssociatedFunctionDecl {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub output_ty: DeclResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}
