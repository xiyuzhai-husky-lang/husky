use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_associated_form_fn_signature(
    db: &dyn SignatureDb,
    decl: TraitAssociatedFunctionDecl,
) -> SignatureResult<TraitAssociatedFormFnSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedFormFnSignature::new(db, todo!()))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitAssociatedFormFnSignature {
    pub return_ty: RawTerm,
}
