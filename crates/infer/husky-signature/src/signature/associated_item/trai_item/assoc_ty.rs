use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitAssociatedTypeSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
