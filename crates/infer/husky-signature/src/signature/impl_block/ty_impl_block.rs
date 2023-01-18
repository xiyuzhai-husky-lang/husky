use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    pub ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
