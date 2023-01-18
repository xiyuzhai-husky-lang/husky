use crate::*;

pub(crate) fn ty_as_trai_associated_ty_signature(db: &dyn SignatureDb, decl: TypeAsTraitAssociatedTypeDecl) -> TypeAsTraitAssociatedTypeSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeAsTraitAssociatedTypeSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitAssociatedTypeSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
