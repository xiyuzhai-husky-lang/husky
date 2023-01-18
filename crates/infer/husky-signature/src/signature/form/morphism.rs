use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn morphism_signature(db: &dyn SignatureDb, decl: MorphismDecl) -> MorphismSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    MorphismSignature::new(
        db,
        ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}



#[salsa::tracked(jar = SignatureJar)]
pub struct MorphismSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl MorphismSignature {}
