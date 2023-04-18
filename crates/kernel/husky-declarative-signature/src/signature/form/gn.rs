use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct GnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates,
}

impl HasDeclarativeSignatureTemplate for GnDecl {
    type DeclarativeSignatureTemplate = GnSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        gn_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub fn gn_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: GnDecl,
) -> DeclarativeSignatureResult<GnSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatureTemplates::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(GnSignature::new(db, implicit_parameters))
}

impl GnSignature {}
