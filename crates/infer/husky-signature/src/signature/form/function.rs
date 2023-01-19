use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn function_signature(db: &dyn SignatureDb, decl: FunctionDecl) -> FunctionSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
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
    FunctionSignature::new(
        db,
        implicit_parameters,
        parameters,
        output_ty,
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct FunctionSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatures,
    #[return_ref]
    pub output_ty: SignatureOutcome<Term>,
    #[return_ref]
    term_sheet: SignatureTermSheet,
}

impl FunctionSignature {}
