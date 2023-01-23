use super::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeImplBlockDecl,
) -> SignatureOutcome<TypeImplBlockSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        term_menu,
    );
    let ty = decl.ty(db);
    let ty = match signature_term_region.expr_term(ty.expr()) {
        Success(ty) => ty,
        Failure(_) => todo!(),
        Abort(_) => todo!(),
    };
    Success(TypeImplBlockSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db),
            signature_term_region,
            term_menu,
        ),
        ty,
    ))
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub ty: Term,
}
