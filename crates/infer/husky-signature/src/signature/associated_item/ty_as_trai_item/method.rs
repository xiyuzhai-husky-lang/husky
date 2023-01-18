use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
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
