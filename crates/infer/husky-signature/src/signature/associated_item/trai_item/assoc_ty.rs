use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_associated_ty_signature(db: &dyn SignatureDb, decl: TraitAssociatedTypeDecl) -> TraitAssociatedTypeSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TraitAssociatedTypeSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitAssociatedTypeSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
