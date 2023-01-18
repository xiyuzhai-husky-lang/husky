use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl TraitSignature {}
