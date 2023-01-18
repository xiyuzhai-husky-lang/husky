use crate::*;

pub(crate) fn trai_associated_function_signature(db: &dyn SignatureDb, decl: TraitAssociatedFunctionDecl) -> TraitAssociatedFunctionSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TraitAssociatedFunctionSignature::new(
        db,
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TraitAssociatedFunctionSignature {
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
