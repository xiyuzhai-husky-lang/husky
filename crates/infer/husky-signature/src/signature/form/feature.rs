use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn feature_signature(
    db: &dyn SignatureDb,
    decl: FeatureDecl,
) -> SignatureResult<FeatureSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => return Err(SignatureError::OutputTypeRawTermError),
        },
        Err(_) => return Err(SignatureError::ExprError),
    };
    Ok(FeatureSignature::new(db, return_ty))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct FeatureSignature {
    pub return_ty: RawTerm,
}
