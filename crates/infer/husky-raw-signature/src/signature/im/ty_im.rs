use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_im_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeImplDecl,
) -> RawSignatureResult<TypeImplRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterRawSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &raw_signature_term_region,
        term_menu,
    );
    let ty = decl.ty(db);
    let ty = match raw_signature_term_region.expr_term(ty.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TypeImplRawSignature::new(
        db,
        ImplicitParameterRawSignatures::from_decl(
            decl.implicit_parameters(db)?,
            raw_signature_term_region,
            term_menu,
        ),
        ty,
    ))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeImplRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
    pub ty: Term,
}
