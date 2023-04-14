use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAssociatedFnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ExplicitParameterSignatures,
    pub return_ty: RawTerm,
}

impl HasSignature for TypeAssociatedFnDecl {
    type Signature = TypeAssociatedFnSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<TypeAssociatedFnSignature> {
        ty_associated_fn_signature(db, self)
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_associated_fn_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedFnDecl,
) -> SignatureResult<TypeAssociatedFnSignature> {
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        signature_term_region,
        raw_term_menu,
    );
    let parameters = ExplicitParameterSignatures::from_decl(
        decl.parameters(db),
        expr_region_data,
        signature_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => signature_term_region.expr_term(return_ty.expr())?,
        None => raw_term_menu.unit(),
    };
    Ok(TypeAssociatedFnSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}
