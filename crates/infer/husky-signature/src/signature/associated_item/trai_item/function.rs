use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TraitAssociatedFunctionSignature {
    pub output_ty: Term,
}
