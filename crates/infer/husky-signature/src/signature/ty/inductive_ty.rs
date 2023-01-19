use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn inductive_ty_signature(
    db: &dyn SignatureDb,
    decl: InductiveTypeDecl,
) -> InductiveTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    InductiveTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct InductiveTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl InductiveTypeSignature {}
