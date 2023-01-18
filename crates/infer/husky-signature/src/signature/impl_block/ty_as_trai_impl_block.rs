use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitImplBlockSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
