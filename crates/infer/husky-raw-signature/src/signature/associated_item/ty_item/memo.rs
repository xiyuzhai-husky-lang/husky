use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_memo_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeMemoDecl,
) -> RawSignatureResult<TypeMemoRawSignature> {
    let im = decl.associated_item(db).im(db);
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let return_ty = match decl.return_ty(db) {
        Ok(return_ty) => match raw_signature_term_region.expr_term(return_ty.expr()) {
            Ok(return_ty) => return_ty,
            Err(_) => return Err(RawSignatureError::OutputTypeTermError),
        },
        Err(_) => return Err(todo!()),
    };
    Ok(TypeMemoRawSignature::new(db, return_ty))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeMemoRawSignature {
    pub return_ty: Term,
}
