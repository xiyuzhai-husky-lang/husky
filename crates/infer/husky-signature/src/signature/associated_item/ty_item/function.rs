use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    #[return_ref]
    pub output_ty: SignatureResult<Term>,
}
