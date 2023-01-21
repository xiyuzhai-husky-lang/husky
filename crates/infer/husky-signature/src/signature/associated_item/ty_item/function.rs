use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedFunctionDecl,
) -> TypeAssociatedFunctionSignature {
    // implementation
    TypeAssociatedFunctionSignature::new(db, todo!())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedFunctionSignature {
    pub output_ty: Term,
}
