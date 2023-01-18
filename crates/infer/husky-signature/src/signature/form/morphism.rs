use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct MorphismSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl MorphismSignature {}
