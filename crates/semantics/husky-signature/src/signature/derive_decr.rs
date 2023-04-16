use super::*;
use husky_decr::DeriveDecr;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct DeriveDecrSignature {
    pub traits: Vec<DeclarativeTerm>,
}

impl HasSignature for DeriveDecr {
    type Signature = DeriveDecrSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        derive_decr_signature(db, self)
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
    let traits = decr
        .traits(db)
        .iter()
        .copied()
        .map(|trai_expr| signature_term_region.expr_term(trai_expr.expr()))
        .collect::<SignatureDeclarativeTermResultBorrowed<Vec<_>>>()?;
    Ok(DeriveDecrSignature::new(db, traits))
}
