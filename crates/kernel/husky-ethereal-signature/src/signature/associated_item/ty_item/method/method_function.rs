use super::*;

// functions are called in functional style, i.e. without parentheses
#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFunctionEtherealSignatureTemplate {}
