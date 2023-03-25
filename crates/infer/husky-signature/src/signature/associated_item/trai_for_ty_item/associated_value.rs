use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_for_ty_associated_value_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedValueDecl,
) -> SignatureResult<TypeAsTraitAssociatedValueSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAsTraitAssociatedValueSignature::new(db))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAsTraitAssociatedValueSignature {}
