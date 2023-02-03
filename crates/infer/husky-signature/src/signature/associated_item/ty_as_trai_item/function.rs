use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub(crate) fn ty_as_trai_associated_function_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitAssociatedFunctionDecl,
) -> SignatureResult<TypeAsTraitAssociatedFunctionSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TypeAsTraitAssociatedFunctionSignature::new(db, todo!()))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAsTraitAssociatedFunctionSignature {
    pub return_ty: Term,
}
