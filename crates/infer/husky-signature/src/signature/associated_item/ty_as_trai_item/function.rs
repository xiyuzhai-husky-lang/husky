use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFunctionSignature {
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
