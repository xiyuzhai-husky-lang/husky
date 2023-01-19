use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_method_signature(
    db: &dyn SignatureDb,
    decl: TypeMethodDecl,
) -> TypeMethodSignature {
    let impl_block = decl.associated_item(db).impl_block(db);
    let parent_symbol_term_registry = db.impl_block_decl(impl_block).ok().map(|decl| {
        impl_block_signature(db, decl)
            .term_sheet(db)
            .symbol_term_registry()
    });
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), parent_symbol_term_registry);
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => match engine.query_new(*output_ty) {
            Some(output_ty) => Success(output_ty),
            None => Abort(SignatureAbortion::TermError),
        },
        Err(_) => Abort(SignatureAbortion::ExprError),
    };
    let parameters = ParameterSignatures::from_decl(decl.parameters(db), &mut engine);
    let implicit_parameters =
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine);
    TypeMethodSignature::new(
        db,
        implicit_parameters,
        parameters,
        output_ty,
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ParameterSignatures,
    #[return_ref]
    pub output_ty: SignatureOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
