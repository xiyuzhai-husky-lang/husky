use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> TypeAssociatedTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_region(db), todo!());
    // implementation
    TypeAssociatedTypeSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedTypeSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
