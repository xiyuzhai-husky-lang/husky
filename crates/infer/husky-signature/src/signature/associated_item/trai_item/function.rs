use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn trai_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedFunctionDecl,
) -> SignatureResult<TraitAssociatedFunctionSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TraitAssociatedFunctionSignature::new(db, todo!()))
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitAssociatedFunctionSignature {
    pub output_ty: Term,
}
