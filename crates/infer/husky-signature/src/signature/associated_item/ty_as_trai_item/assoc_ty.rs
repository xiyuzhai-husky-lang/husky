use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedTypeSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
