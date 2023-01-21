use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeMemoDecl {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub output_ty: DeclResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}
