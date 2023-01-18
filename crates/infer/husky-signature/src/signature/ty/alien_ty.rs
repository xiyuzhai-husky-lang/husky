use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn alien_ty_signature(db: &dyn SignatureDb, decl: AlienTypeDecl) -> AlienTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    AlienTypeSignature::new(
        db,
        ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct AlienTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl AlienTypeSignature {}
