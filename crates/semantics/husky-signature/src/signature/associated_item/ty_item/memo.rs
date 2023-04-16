use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_memo_signature(
    db: &dyn SignatureDb,
    decl: TypeMemoDecl,
) -> SignatureResult<TypeMemoSignature> {
    let _im = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let memo_ty = match decl.memo_ty(db) {
        Some(memo_ty) => signature_term_region.expr_term(memo_ty.expr())?,
        None => raw_term_menu.unit(),
    };
    Ok(TypeMemoSignature::new(db, memo_ty))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeMemoSignature {
    pub return_ty: DeclarativeTerm,
}
