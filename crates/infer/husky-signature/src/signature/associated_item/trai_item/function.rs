use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitAssociatedFunctionSignature {
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
