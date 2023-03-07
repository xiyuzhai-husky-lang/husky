use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedFunctionDecl,
) -> SignatureResult<TypeAssociatedFunctionSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAssociatedFunctionSignature::new(db, todo!()))
}

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    pub return_ty: RawTerm,
}
