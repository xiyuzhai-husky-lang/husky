use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedFunctionDecl,
) -> SignatureResult<TypeAssociatedFunctionSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TypeAssociatedFunctionSignature::new(db, todo!()))
}

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    pub output_ty: Term,
}
