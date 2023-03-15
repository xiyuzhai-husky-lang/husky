use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAssociatedFnSignature {
    pub return_ty: RawTerm,
}

#[salsa::tracked(jar = SignatureJar)]
pub fn ty_associated_fn_signature(
    db: &dyn SignatureDb,
    decl: TypeAssociatedFnDecl,
) -> SignatureResult<TypeAssociatedFnSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAssociatedFnSignature::new(db, todo!()))
}
