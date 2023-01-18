use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_method_signature(db: &dyn SignatureDb, decl: TypeMethodDecl) -> TypeMethodSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeMethodSignature::new(
        db,
        todo!(),
        todo!(),
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}


#[salsa::tracked(jar = SignatureJar)]
pub struct TypeMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
