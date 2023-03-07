use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedTypeDecl,
) -> SignatureResult<TraitAssociatedTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedTypeSignature::new(db))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitAssociatedTypeSignature {}
