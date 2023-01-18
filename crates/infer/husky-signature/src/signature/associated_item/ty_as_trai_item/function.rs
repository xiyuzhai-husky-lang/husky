use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFunctionSignature {
    pub output_ty: Term,
}
