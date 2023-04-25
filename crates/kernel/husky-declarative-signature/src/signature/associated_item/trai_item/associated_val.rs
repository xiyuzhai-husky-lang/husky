use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_associated_val_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitAssociatedValDecl,
) -> DeclarativeSignatureResult<TraitAssociatedValDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedValDeclarativeSignatureTemplate::new(db))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitAssociatedValDeclarativeSignatureTemplate {}
