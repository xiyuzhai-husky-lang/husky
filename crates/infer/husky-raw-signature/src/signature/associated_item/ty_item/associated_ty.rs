use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_associated_ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> RawSignatureResult<TypeAssociatedTypeRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TypeAssociatedTypeRawSignature::new(db))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeAssociatedTypeRawSignature {}
