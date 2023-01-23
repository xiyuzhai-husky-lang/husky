use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn feature_signature(
    db: &dyn SignatureDb,
    decl: FeatureDecl,
) -> SignatureResult<FeatureSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let output_ty = match decl.output_ty(db) {
        Ok(output_ty) => match signature_term_region.expr_term(output_ty.expr()) {
            Ok(output_ty) => output_ty,
            Err(_) => return Err(SignatureError::OutputTypeTermError),
        },
        Err(_) => return Err(SignatureError::ExprError),
    };
    Ok(FeatureSignature::new(db, output_ty))
}

#[salsa::interned(jar = SignatureJar)]
pub struct FeatureSignature {
    pub output_ty: Term,
}
