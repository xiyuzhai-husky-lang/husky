use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedTypeDeclarativeSignature {}

pub(crate) fn ty_associated_ty_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> DeclarativeSignatureResult<TypeAssociatedTypeDeclarativeSignature> {
    todo!()
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_associated_ty_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeAssociatedTypeDecl,
) -> DeclarativeSignatureResult<TypeAssociatedTypeDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAssociatedTypeDeclarativeSignature::new(db))
}
