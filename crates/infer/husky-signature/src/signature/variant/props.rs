use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct PropsVariantSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
