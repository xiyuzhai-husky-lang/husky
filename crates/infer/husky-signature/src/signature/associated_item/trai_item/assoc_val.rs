use crate::*;

pub(crate) fn trai_associated_val_signature(db: &dyn SignatureDb, decl: TraitAssociatedValueDecl) -> TraitAssociatedValueSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TraitAssociatedValueSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitAssociatedValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
