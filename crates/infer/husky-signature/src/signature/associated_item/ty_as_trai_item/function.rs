use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_function_signature(db: &dyn SignatureDb, decl: TypeAsTraitAssociatedFunctionDecl) -> TypeAsTraitAssociatedFunctionSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeAsTraitAssociatedFunctionSignature::new(
        db,
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFunctionSignature {
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
