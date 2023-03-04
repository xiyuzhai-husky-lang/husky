use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_method_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeMethodDecl,
) -> RawSignatureResult<TypeMethodRawSignature> {
    let im = decl.associated_item(db).im(db);
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();

    let implicit_parameters = ImplicitParameterRawSignatures::from_decl(
        decl.implicit_parameters(db)?,
        raw_signature_term_region,
        raw_term_menu,
    );

    let parameters =
        RegularParameterRawSignatures::from_decl(decl.parameters(db)?, raw_signature_term_region)?;
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match raw_signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => todo!(),
        },
        Err(_) => todo!(), //  Err(RawSignatureRawTermAbortion::ExprError),
    };
    Ok(TypeMethodRawSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}

#[salsa::tracked(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeMethodRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
    #[return_ref]
    pub parameters: RegularParameterRawSignatures,
    pub return_ty: RawTerm,
}
