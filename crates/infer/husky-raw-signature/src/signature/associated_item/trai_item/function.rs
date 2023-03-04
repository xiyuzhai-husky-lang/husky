use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn trai_associated_function_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TraitAssociatedFunctionDecl,
) -> RawSignatureResult<TraitAssociatedFunctionRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(TraitAssociatedFunctionRawSignature::new(db, todo!()))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TraitAssociatedFunctionRawSignature {
    pub return_ty: RawTerm,
}
