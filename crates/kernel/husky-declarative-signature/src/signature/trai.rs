use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct TraitDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates,
}

impl TraitDeclarativeSignatureTemplate {}

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub fn trai_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitDecl,
) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatureTemplates::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(TraitDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
    ))
}
