use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_memo_signature(db: &dyn SignatureDb, decl: TypeMemoDecl) -> TypeMemoSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), todo!());
    // implementation
    TypeMemoSignature::new(
        db,
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeMemoSignature {
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
