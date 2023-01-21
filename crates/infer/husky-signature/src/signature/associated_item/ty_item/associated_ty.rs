use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> TypeAssociatedTypeSignature {
    // implementation
    TypeAssociatedTypeSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAssociatedTypeSignature {}
