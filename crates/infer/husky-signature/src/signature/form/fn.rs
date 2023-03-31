use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct FnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ExplicitParameterSignatures,
    pub return_ty: RawTerm,
}

impl HasSignature for FnDecl {
    type Signature = FnSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        fn_signature(db, self)
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub fn fn_signature(db: &dyn SignatureDb, decl: FnDecl) -> SignatureResult<FnSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        signature_term_region,
        raw_term_menu,
    );
    let parameters =
        ExplicitParameterSignatures::from_decl(decl.parameters(db), signature_term_region)?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => signature_term_region.expr_term(return_ty.expr())?,
        None => raw_term_menu.unit(),
    };
    Ok(FnSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}

impl FnSignature {}
