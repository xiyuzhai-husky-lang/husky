use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedFunctionDecl,
) -> SignatureResult<TypeAsTraitAssociatedFnSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAsTraitAssociatedFnSignature::new(db, todo!()))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFnSignature {
    pub return_ty: RawTerm,
}
