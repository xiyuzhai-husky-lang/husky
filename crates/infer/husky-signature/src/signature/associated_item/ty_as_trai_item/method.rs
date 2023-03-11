use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_method_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitMethodDecl,
) -> SignatureResult<TypeAsTraitMethodSignature> {
    let impl_block = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    todo!()
    // let return_ty = match decl.return_ty(db) {
    //     Ok(return_ty) => engine.query_new(*return_ty),
    //     Err(_) =>  Err(SignatureRawTermAbortion::ExprError),
    // };
    // let parameters = ParameterSignatures::from_decl(decl.parameters(db), signature_term_region);
    // let implicit_parameters =
    //     ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), signature_term_region);
    // TypeAsTraitMethodSignature::new(db, implicit_parameters, parameters, return_ty)
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: RegularParameterSignatures,
    pub return_ty: RawTerm,
}
