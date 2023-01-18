use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    pub ty: Term,
}
