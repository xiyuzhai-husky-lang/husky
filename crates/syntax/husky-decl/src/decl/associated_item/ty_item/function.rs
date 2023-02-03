use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedFunctionDecl {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}
