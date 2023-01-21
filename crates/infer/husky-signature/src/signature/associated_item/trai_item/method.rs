use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_method_signature(
    db: &dyn SignatureDb,
    decl: TraitMethodDecl,
) -> TraitMethodSignature {
    // implementation
    TraitMethodSignature::new(db, todo!())
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitMethodSignature {
    pub output_ty: Term,
}
