use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_associated_ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitAssociatedTypeSynDecl,
) -> DeclarativeSignatureResult<TraitAssociatedTypeDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let _declarative_term_region = declarative_term_region(db, syn_expr_region);
    let _declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    Ok(TraitAssociatedTypeDeclarativeSignatureTemplate::new(db))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitAssociatedTypeDeclarativeSignatureTemplate {}
