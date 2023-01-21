use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedTypeDecl,
) -> TraitAssociatedTypeSignature {
    // implementation
    TraitAssociatedTypeSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitAssociatedTypeSignature {}
