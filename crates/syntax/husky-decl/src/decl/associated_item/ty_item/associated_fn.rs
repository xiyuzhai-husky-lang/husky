use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedFnDecl {
    #[id]
    pub id: AssociatedItemId,
    pub path: Option<TypeItemPath>,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    parameter_decl_list: DeclExprResult<ExplicitParameterDeclList>,
    #[return_ref]
    pub curry_token: DeclExprResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclExprResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
}
