use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn trai_signature(db: &dyn SignatureDb, decl: TraitDecl) -> TraitSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_region(db), None);
    // implementation
    TraitSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl TraitSignature {}
