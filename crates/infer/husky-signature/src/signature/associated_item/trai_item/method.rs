use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitMethodSignature {
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
