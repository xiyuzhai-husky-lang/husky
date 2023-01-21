use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedValueDecl,
) -> TypeAssociatedValueSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_region(db), todo!());
    // implementation
    TypeAssociatedValueSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAssociatedValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
