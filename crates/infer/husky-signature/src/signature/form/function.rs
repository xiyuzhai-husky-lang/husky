use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct FunctionSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
}

impl FunctionSignature {}
