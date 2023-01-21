use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedValueDecl,
) -> TraitAssociatedValueSignature {
    // implementation
    TraitAssociatedValueSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitAssociatedValueSignature {}
