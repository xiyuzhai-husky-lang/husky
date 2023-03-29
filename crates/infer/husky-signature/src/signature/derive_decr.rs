use super::*;
use husky_decr::DeriveDecr;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct DeriveDecrSignature {}

impl HasSignature for DeriveDecr {
    type Signature = DeriveDecrSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        todo!()
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub fn derive_decr_signature(
    db: &dyn SignatureDb,
    decr: DeriveDecr,
) -> SignatureResult<DeriveDecrSignature> {
    let expr_region = decr.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    todo!();
    Ok(DeriveDecrSignature::new(db))
}
