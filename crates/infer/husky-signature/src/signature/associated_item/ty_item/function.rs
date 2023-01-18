use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
}
