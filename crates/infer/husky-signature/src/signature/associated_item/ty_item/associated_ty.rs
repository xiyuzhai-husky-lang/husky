use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> SignatureResult<TypeAssociatedTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(TypeAssociatedTypeSignature::new(db))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAssociatedTypeSignature {}
