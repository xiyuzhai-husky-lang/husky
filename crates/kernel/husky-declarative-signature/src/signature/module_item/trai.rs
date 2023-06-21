use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
}

impl TraitDeclarativeSignatureTemplate {}

impl HasDeclarativeSignatureTemplate for TraitPath {
    type DeclarativeSignatureTemplate = TraitDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn trai_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TraitPath,
) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(TraitDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
    ))
}
