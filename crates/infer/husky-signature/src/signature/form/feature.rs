use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct FeatureSignature {
    pub return_ty: RawTerm,
}

impl HasSignature for FeatureDecl {
    type Signature = FeatureSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        feature_signature(db, self)
    }
}

impl FeatureSignature {
    #[inline(always)]
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        &[]
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub fn feature_signature(
    db: &dyn SignatureDb,
    decl: FeatureDecl,
) -> SignatureResult<FeatureSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => return Err(SignatureError::ReturnTypeRawTermError),
        },
        Err(_) => return Err(SignatureError::ExprError),
    };
    Ok(FeatureSignature::new(db, return_ty))
}
