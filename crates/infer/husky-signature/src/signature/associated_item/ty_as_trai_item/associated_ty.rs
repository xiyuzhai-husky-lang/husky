use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_associated_ty_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedTypeDecl,
) -> SignatureResult<TypeAsTraitAssociatedTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAsTraitAssociatedTypeSignature::new(db))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAsTraitAssociatedTypeSignature {}
