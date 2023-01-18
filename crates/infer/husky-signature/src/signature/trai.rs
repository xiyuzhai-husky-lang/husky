use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl TraitSignature {}
