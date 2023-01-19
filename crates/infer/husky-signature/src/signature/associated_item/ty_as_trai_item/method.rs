use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_method_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitMethodDecl,
) -> TypeAsTraitMethodSignature {
    let impl_block = decl.associated_item(db).impl_block(db);
    let parent_term_symbol_page = db.impl_block_decl(impl_block).ok().map(|decl| {
        impl_block_signature(db, decl)
            .term_sheet(db)
            .term_symbol_page()
    });
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), parent_term_symbol_page);
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => engine.query_new(*output_ty),
        Err(_) => Abort(SignatureTermAbortion::ExprError),
    };
    let parameters = ParameterSignatures::from_decl(decl.parameters(db), &mut engine);
    let implicit_parameters =
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine);
    TypeAsTraitMethodSignature::new(
        db,
        implicit_parameters,
        parameters,
        output_ty,
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ParameterSignatures,
    #[return_ref]
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
