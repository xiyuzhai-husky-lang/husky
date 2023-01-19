use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitImplBlockDecl,
) -> TypeAsTraitImplBlockSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
    // implementation
    TypeAsTraitImplBlockSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAsTraitImplBlockSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
