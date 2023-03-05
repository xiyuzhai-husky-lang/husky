use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn trai_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedFunctionDecl,
) -> SignatureResult<TraitAssociatedFunctionSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedFunctionSignature::new(db, todo!()))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitAssociatedFunctionSignature {
    pub return_ty: RawTerm,
}
