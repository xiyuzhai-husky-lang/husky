use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn feature_signature(
    db: &dyn SignatureDb,
    decl: FeatureDecl,
) -> SignatureOutcome<FeatureSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Success(FeatureSignature::new(db))
}

#[salsa::interned(jar = SignatureJar)]
pub struct FeatureSignature {}
