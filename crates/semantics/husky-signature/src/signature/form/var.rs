use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct VarSignature {
    pub return_ty: DeclarativeTerm,
}

impl HasSignature for FeatureDecl {
    type Signature = VarSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        var_signature(db, self)
    }
}

impl VarSignature {
    #[inline(always)]
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        &[]
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub fn var_signature(db: &dyn SignatureDb, decl: FeatureDecl) -> SignatureResult<VarSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let var_ty = match decl.var_ty(db) {
        Some(var_ty) => signature_term_region.expr_term(var_ty.expr())?,
        None => raw_term_menu.unit(),
    };
    Ok(VarSignature::new(db, var_ty))
}
