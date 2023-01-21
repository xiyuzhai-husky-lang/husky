use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedTypeDecl,
) -> TypeAsTraitAssociatedTypeSignature {
    // implementation
    TypeAsTraitAssociatedTypeSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedTypeSignature {}
