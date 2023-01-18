use crate::*;

pub(crate) fn ty_associated_function_signature(db: &dyn SignatureDb, decl: TypeAssociatedFunctionDecl) -> TypeAssociatedFunctionSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeAssociatedFunctionSignature::new(
        db,
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
