use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TraitMethodSignature {
    pub output_ty: Term,
}
