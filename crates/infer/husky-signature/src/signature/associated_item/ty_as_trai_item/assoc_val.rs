use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
