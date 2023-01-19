use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn morphism_signature(db: &dyn SignatureDb, decl: MorphismDecl) -> MorphismSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
    // implementation
    MorphismSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct MorphismSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl MorphismSignature {}
