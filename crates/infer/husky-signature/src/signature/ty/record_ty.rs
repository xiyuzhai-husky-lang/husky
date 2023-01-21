use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn record_ty_signature(db: &dyn SignatureDb, decl: RecordTypeDecl) -> RecordTypeSignature {
    RecordTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct RecordTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl RecordTypeSignature {}
