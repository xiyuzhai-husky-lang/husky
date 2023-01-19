use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn feature_signature(db: &dyn SignatureDb, decl: FeatureDecl) -> FeatureSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
    // implementation
    FeatureSignature::new(db, engine.finish())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct FeatureSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
