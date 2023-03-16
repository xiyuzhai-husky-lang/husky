use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct FormFnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: RegularParameterSignatures,
    pub return_ty: RawTerm,
}

#[salsa::tracked(jar = SignatureJar)]
pub fn form_fn_signature(
    db: &dyn SignatureDb,
    decl: FormFnDecl,
) -> SignatureResult<FormFnSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db)?,
        signature_term_region,
        raw_term_menu,
    );
    let parameters =
        RegularParameterSignatures::from_decl(decl.parameters(db)?, signature_term_region)?;
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => todo!(),
        },
        Err(_) => return Err(SignatureError::ExprError),
    };
    Ok(FormFnSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}

impl FormFnSignature {}
