use super::*;
use husky_decr::DeriveDecr;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct DeriveDecrDeclarativeSignature {
    pub traits: Vec<DeclarativeTerm>,
}

impl HasDeclarativeSignature for DeriveDecr {
    type DeclarativeSignature = DeriveDecrDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        derive_decr_declarative_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn derive_decr_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decr: DeriveDecr,
) -> DeclarativeSignatureResult<DeriveDecrDeclarativeSignature> {
    let expr_region = decr.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let traits = decr
        .traits(db)
        .iter()
        .copied()
        .map(|trai_expr| declarative_term_region.expr_term(trai_expr.expr()))
        .collect::<DeclarativeTermResultBorrowed2<Vec<_>>>()?;
    Ok(DeriveDecrDeclarativeSignature::new(db, traits))
}
