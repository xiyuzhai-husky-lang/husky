use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeMethodSignature {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    #[return_ref]
    pub curry_token: SignatureResult<CurryToken>,
    #[return_ref]
    pub output_ty: SignatureResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: SignatureResult<EolColonToken>,
}
