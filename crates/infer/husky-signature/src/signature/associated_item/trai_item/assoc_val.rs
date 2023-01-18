use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitAssociatedValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
