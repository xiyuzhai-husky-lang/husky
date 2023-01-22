use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn function_signature(db: &dyn SignatureDb, decl: FunctionDecl) -> FunctionSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    todo!()
    // let output_ty = match decl.output_ty(db) {
    //     Ok(output_ty) => engine.query_new(*output_ty),
    //     Err(_) => Abort(SignatureTermAbortion::ExprError),
    // };
    // let parameters = ParameterSignatures::from_decl(decl.parameters(db), signature_term_region);
    // let implicit_parameters =
    //     ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), signature_term_region);
    // FunctionSignature::new(
    //     db,
    //     implicit_parameters,
    //     parameters,
    //     output_ty,
    //     engine.finish(),
    // )
}

#[salsa::interned(jar = SignatureJar)]
pub struct FunctionSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameter_decl_list: ParameterSignatures,
    pub output_ty: Term,
}

impl FunctionSignature {}
