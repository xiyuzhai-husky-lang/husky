use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn function_signature(db: &dyn SignatureDb, decl: FunctionDecl) -> FunctionSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_region(db), None);
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => engine.query_new(*output_ty),
        Err(_) => Abort(SignatureTermAbortion::ExprError),
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
    pub output_ty: SignatureTermOutcome<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl FunctionSignature {}
