use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn record_ty_signature(db: &dyn SignatureDb, decl: RecordTypeDecl) -> RecordTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_region(db), None);
    RecordTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct RecordTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl RecordTypeSignature {}
