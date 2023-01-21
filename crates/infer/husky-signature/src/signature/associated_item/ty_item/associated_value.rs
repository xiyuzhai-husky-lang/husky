use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedValueDecl,
) -> TypeAssociatedValueSignature {
    // implementation
    TypeAssociatedValueSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAssociatedValueSignature {}
