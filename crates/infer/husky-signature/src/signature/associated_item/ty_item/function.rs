use husky_entity_tree::AssociatedItem;

use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    #[id]
    pub path: Option<TypeItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub curry_token: SignatureResult<CurryToken>,
    #[return_ref]
    pub output_ty: SignatureResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: SignatureResult<EolColonToken>,
}
