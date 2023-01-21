use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedValueDecl,
) -> TypeAsTraitAssociatedValueSignature {
    // implementation
    TypeAsTraitAssociatedValueSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedValueSignature {}
