use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_method_signature(db: &dyn SignatureDb, decl: TypeAsTraitMethodDecl) -> TypeAsTraitMethodSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeAsTraitMethodSignature::new(
        db,
        todo!(),
        todo!(),
        todo!(),
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatureList,
    pub output_ty: Term,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
