use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_memo_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeMemoizedFieldDecl,
) -> DeclarativeSignatureResult<TypeMemoizedFieldDeclarativeSignatureTemplate> {
    let _im = decl.associated_item(db).impl_block(db);
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let memo_ty = match decl.memo_ty(db) {
        Some(memo_ty) => declarative_term_region.expr_term(memo_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TypeMemoizedFieldDeclarativeSignatureTemplate::new(
        db, memo_ty,
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeMemoizedFieldDeclarativeSignatureTemplate {
    pub return_ty: DeclarativeTerm,
}
