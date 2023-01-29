use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn trai_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedValueDecl,
) -> SignatureResult<TraitAssociatedValueSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TraitAssociatedValueSignature::new(db))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitAssociatedValueSignature {}
