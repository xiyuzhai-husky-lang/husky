use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAssociatedTypeSignature {}

pub(crate) fn ty_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> SignatureResult<TypeAssociatedTypeSignature> {
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_ty_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> SignatureResult<TypeAssociatedTypeSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAssociatedTypeSignature::new(db))
}
