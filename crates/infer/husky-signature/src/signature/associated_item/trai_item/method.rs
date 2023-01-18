use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitMethodSignature {
    pub output_ty: Term,
}
