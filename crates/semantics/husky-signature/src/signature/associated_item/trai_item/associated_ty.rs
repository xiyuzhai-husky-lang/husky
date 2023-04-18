use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_associated_ty_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitAssociatedTypeDecl,
) -> DeclarativeSignatureResult<TraitAssociatedTypeDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedTypeDeclarativeSignature::new(db))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitAssociatedTypeDeclarativeSignature {}
