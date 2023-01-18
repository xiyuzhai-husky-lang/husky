use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFunctionSignature {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub curry_token: SignatureResult<CurryToken>,
    #[return_ref]
    pub output_ty: SignatureResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: SignatureResult<EolColonToken>,
}
