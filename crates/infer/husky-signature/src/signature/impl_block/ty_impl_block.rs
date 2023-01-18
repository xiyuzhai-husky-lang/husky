use super::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    pub ty: Term,
}
