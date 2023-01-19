use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_value_signature(db: &dyn SignatureDb, decl: TypeAsTraitAssociatedValueDecl) -> TypeAsTraitAssociatedValueSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeAsTraitAssociatedValueSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
