use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterSignatureList>,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    pub output_ty: Term,
}
