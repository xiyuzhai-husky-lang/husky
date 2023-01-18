use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn trai_signature(db: &dyn SignatureDb, decl: TraitDecl) -> TraitSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TraitSignature::new(
        db,
        ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}


impl TraitSignature {}
