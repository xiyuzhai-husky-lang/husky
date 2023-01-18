use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn record_ty_signature(db: &dyn SignatureDb, decl: RecordTypeDecl) -> RecordTypeSignature {
    // implementation
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct RecordTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl RecordTypeSignature {}
