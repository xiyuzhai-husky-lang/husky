use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_as_trai_method_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitMethodDecl,
) -> SignatureResult<TypeAsTraitMethodSignature> {
    let impl_block = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    todo!()
    // let output_ty = match decl.output_ty(db) {
    //     Ok(output_ty) => engine.query_new(*output_ty),
    //     Err(_) =>  Err(SignatureTermAbortion::ExprError),
    // };
    // let parameters = ParameterSignatures::from_decl(decl.parameters(db), signature_term_region);
    // let implicit_parameters =
    //     ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), signature_term_region);
    // TypeAsTraitMethodSignature::new(db, implicit_parameters, parameters, output_ty)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitMethodSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ParameterSignatures,
    pub output_ty: Term,
}
