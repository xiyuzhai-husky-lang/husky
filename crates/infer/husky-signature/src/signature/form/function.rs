use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn function_signature(
    db: &dyn SignatureDb,
    decl: FunctionDecl,
) -> SignatureResult<FunctionSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();

    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        signature_term_region,
        term_menu,
    );

    let parameters = ParameterSignatures::from_decl(decl.parameters(db), signature_term_region)?;
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => match signature_term_region.expr_term(output_ty.expr()) {
            Ok(output_ty) => output_ty,
            Err(_) => todo!(),
        },
        Err(_) => return Err(SignatureError::ExprError),
    };
    Ok(FunctionSignature::new(
        db,
        implicit_parameters,
        parameters,
        output_ty,
    ))
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
