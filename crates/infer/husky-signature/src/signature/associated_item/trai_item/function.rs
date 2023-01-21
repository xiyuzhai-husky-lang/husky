use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedFunctionDecl,
) -> TraitAssociatedFunctionSignature {
    // implementation
    TraitAssociatedFunctionSignature::new(db, todo!())
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitAssociatedFunctionSignature {
    pub output_ty: Term,
}
