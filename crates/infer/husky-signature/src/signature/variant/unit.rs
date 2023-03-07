use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct UnitVariantSignature {}
