use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
