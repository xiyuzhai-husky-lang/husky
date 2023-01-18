use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeMemoSignature {
    #[return_ref]
    pub output_ty: SignatureResult<Term>,
}
