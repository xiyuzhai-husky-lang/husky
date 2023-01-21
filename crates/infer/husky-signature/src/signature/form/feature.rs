use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn feature_signature(db: &dyn SignatureDb, decl: FeatureDecl) -> FeatureSignature {
    // implementation
    FeatureSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct FeatureSignature {}
