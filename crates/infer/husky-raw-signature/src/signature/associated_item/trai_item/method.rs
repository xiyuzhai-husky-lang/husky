use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn trai_method_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TraitMethodDecl,
) -> RawSignatureResult<TraitMethodRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(TraitMethodRawSignature::new(db, todo!()))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TraitMethodRawSignature {
    pub return_ty: RawTerm,
}
