use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_as_trai_method_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeAsTraitMethodDecl,
) -> RawSignatureResult<TypeAsTraitMethodRawSignature> {
    let im = decl.associated_item(db).im(db);
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    todo!()
    // let return_ty = match decl.return_ty(db) {
    //     Ok(return_ty) => engine.query_new(*return_ty),
    //     Err(_) =>  Err(RawSignatureRawTermAbortion::ExprError),
    // };
    // let parameters = ParameterRawSignatures::from_decl(decl.parameters(db), raw_signature_term_region);
    // let implicit_parameters =
    //     ImplicitParameterRawSignatures::from_decl(decl.implicit_parameters(db), raw_signature_term_region);
    // TypeAsTraitMethodRawSignature::new(db, implicit_parameters, parameters, return_ty)
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeAsTraitMethodRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
    #[return_ref]
    pub parameters: RegularParameterRawSignatures,
    pub return_ty: RawTerm,
}
