use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_memo_signature(db: &dyn SignatureDb, decl: TypeMemoDecl) -> TypeMemoSignature {
    let impl_block = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    todo!()
    // let output_ty = match decl.output_ty(db) {
    //     Ok(output_ty) => engine.query_new(*output_ty),
    //     Err(_) => Abort(SignatureTermAbortion::ExprError),
    // };
    // TypeMemoSignature::new(db, output_ty, engine.finish())
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeMemoSignature {
    pub output_ty: Term,
}
