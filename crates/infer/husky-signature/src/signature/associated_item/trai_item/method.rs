use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn trai_method_signature(
    db: &dyn SignatureDb,
    decl: TraitMethodDecl,
) -> SignatureResult<TraitMethodSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TraitMethodSignature::new(db, todo!()))
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitMethodSignature {
    pub output_ty: Term,
}
