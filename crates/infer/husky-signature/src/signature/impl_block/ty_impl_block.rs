use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeImplBlockDecl,
) -> TypeImplBlockSignature {
    todo!()
    // let ty = decl.ty(db);
    // let ty = engine.query_new(ty);
    // TypeImplBlockSignature::new(
    //     db,
    //     ty,
    //     // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
    //     engine.finish(),
    // )
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    pub ty: Term,
}
