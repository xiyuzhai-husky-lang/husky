use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedFunctionDecl,
) -> TypeAsTraitAssociatedFunctionSignature {
    // implementation
    TypeAsTraitAssociatedFunctionSignature::new(db, todo!())
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFunctionSignature {
    pub output_ty: Term,
}
