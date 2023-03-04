use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_associated_function_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeAssociatedFunctionDecl,
) -> RawSignatureResult<TypeAssociatedFunctionRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TypeAssociatedFunctionRawSignature::new(db, todo!()))
}

#[salsa::tracked(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeAssociatedFunctionRawSignature {
    pub return_ty: Term,
}
