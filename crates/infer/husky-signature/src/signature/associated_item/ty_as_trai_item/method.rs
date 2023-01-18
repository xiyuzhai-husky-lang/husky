use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
