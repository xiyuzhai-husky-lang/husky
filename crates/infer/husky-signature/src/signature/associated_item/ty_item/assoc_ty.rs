use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedTypeSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
