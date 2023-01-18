use super::*;

pub(crate) fn ty_impl_block_signature(db: &dyn SignatureDb, decl: TypeImplBlockDecl) -> TypeImplBlockSignature{
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    let ty = decl.ty(db);
    let ty = engine.query_new(ty);
    TypeImplBlockSignature::new(
        db,
        ty,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}


#[salsa::tracked(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    pub ty: Option<Term>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
