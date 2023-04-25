use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct GnDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
}

impl HasDeclarativeSignatureTemplate for GnDecl {
    type DeclarativeSignatureTemplate = GnDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        gn_declarative_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn gn_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: GnDecl,
) -> DeclarativeSignatureResult<GnDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(GnDeclarativeSignatureTemplate::new(db, implicit_parameters))
}

impl GnDeclarativeSignatureTemplate {}
